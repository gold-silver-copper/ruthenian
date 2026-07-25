"""Every Ruthenian measurement, computed over the WHOLE dump.

Input is `ru_all.jsonl`, produced by one full scan of raw-wiktextract-data.jsonl:

    LC_ALL=C grep -F '"lang_code": "ru"' raw-wiktextract-data.jsonl > ru_all.jsonl

That filter is a superset — every record whose lang_code is ru contains the
literal — so measuring from it is measuring from the whole dump. No windows, no
`dd`, no sampling of any kind.
"""

import json, sys, collections, re, random, io

STRESS = '́'
def bare(s): return s.replace(STRESS, '')

QUAL = {'dated','obsolete','archaic','colloquial','rare','irregular','literary',
        'poetic','proscribed','nonstandard','informal','regional'}

def slot_of(f):
    t = [x for x in f.get('tags', [])
         if x not in ('table-tags','class','inflection-template','romanization','canonical')]
    if any(x in QUAL for x in t): return None
    return ' '.join(sorted(t))

def forms_map(d, source):
    m = {}
    for f in d.get('forms', []):
        if 'romanization' in f.get('tags', []): continue
        if f.get('source') != source and 'canonical' not in f.get('tags', []): continue
        s = slot_of(f)
        if s and s not in m: m[s] = f.get('form')
    return m

def classes_of(d):
    return [f['form'] for f in d.get('forms', []) if 'class' in f.get('tags', [])]

def detect_mutation(lemma, fm):
    one = fm.get('first-person present singular') or fm.get('first-person future singular')
    if not one or one == '-' or ' ' in one: return None
    inf, o = bare(lemma), bare(one)
    if len(inf) < 4 or inf[-3:] not in ('ить','ать','еть','ять','оть','уть'): return None
    stem = inf[:-3]
    if not stem: return None
    if o.startswith(stem + 'л'): return f'{stem[-1]} -> {stem[-1]}л'
    if o.startswith(stem): return None
    i = 0
    while i < min(len(stem), len(o)) and stem[i] == o[i]: i += 1
    a, b = stem[i:], re.sub('(у|ю)$', '', o[i:])
    if a and len(a) <= 2 and len(b) <= 3: return f'{a} -> {b}'
    return None

codes = collections.Counter()
gap_by_aspect = collections.Counter(); verbs_by_aspect = collections.Counter()
gap_slots = collections.Counter()
muts = collections.Counter(); mut_ex = collections.defaultdict(list)
ppp = collections.Counter()
noun_stem = collections.Counter(); noun_accent = collections.Counter()
adj_kind = collections.Counter()
verb_accent = collections.Counter()
lab_no_epen = collections.Counter()

verbs, nouns, adjs = [], [], []
n_ru = 0

for line in open(sys.argv[1], encoding='utf-8'):
    try: d = json.loads(line)
    except Exception: continue
    if d.get('lang_code') != 'ru': continue
    n_ru += 1
    pos, w = d.get('pos'), d.get('word') or ''
    ht = (d.get('head_templates') or [{}])[0]
    it = (d.get('inflection_templates') or [{}])[0]
    single = w and ' ' not in w and not w.startswith('-')

    if pos == 'verb' and it.get('name') == 'ru-conj':
        args = it.get('args', {})
        code = args.get('2', '')
        codes[code] += 1
        aspect = args.get('1', '?')
        verbs_by_aspect[aspect] += 1
        m = re.search(r'([a-f])', code)
        verb_accent[m.group(1) if m else 'none'] += 1
        fm = forms_map(d, 'conjugation')
        for slot, form in fm.items():
            if form == '-':
                gap_by_aspect[aspect] += 1
                gap_slots[slot] += 1
        has_ppp = fm.get('participle passive past') not in (None, '-')
        ppp[('+p' in code, has_ppp)] += 1
        mu = detect_mutation(w, fm)
        if mu:
            muts[mu] += 1
            if len(mut_ex[mu]) < 2: mut_ex[mu].append(f'{bare(w)}/{bare(fm.get("first-person present singular") or fm.get("first-person future singular"))}')
        # labial-final stem in class 1: does it take epenthesis?
        b = bare(w)
        if code.startswith('1') and len(b) > 4 and b[-3:] in ('ать','ить','еть') and b[-4] in 'бпвфм':
            one = fm.get('first-person present singular') or ''
            lab_no_epen['epenthesis' if bare(one).startswith(b[:-3] + 'л') else 'no epenthesis'] += 1
        if single and len(fm) >= 8: verbs.append((w, d, code))

    elif pos == 'noun' and ht.get('name', '').startswith('ru-noun'):
        cl = classes_of(d)
        for c in cl:
            if 'stem' in c: noun_stem[c] += 1
            elif c.startswith('accent'): noun_accent[c] += 1
        fm = forms_map(d, 'declension')
        if single and len(fm) >= 8: nouns.append((w, d, '|'.join(cl)))

    elif pos == 'adj' and ht.get('name') == 'ru-adj':
        b = bare(w)
        if b.endswith('ий'):
            stem = b[:-2]
            adj_kind['velar/sibilant (hard endings, i-spelling)' if stem and stem[-1] in 'кгхжшчщ'
                     else 'true soft (-nij)'] += 1
        elif b.endswith('ой'): adj_kind['stressed -oj (hard)'] += 1
        elif b.endswith('ый'): adj_kind['hard -yj'] += 1
        else: adj_kind['other'] += 1
        fm = forms_map(d, 'declension')
        if single and len(fm) >= 6: adjs.append((w, d, (classes_of(d) or ['?'])[0]))

out = {}
print(f'ru records scanned: {n_ru}')
print(f'verb lemmas with ru-conj: {sum(codes.values())}   distinct class codes: {len(codes)}')
cov = sum(v for k, v in codes.items() if re.match(r'^[1-6](?!\d)', k))
print(f'classes 1-6: {cov}/{sum(codes.values())} = {100*cov/sum(codes.values()):.1f}%')
print()
print('== gaps by aspect ==')
for k in sorted(verbs_by_aspect, key=lambda x: -verbs_by_aspect[x]):
    print(f'   {k:<18} verbs {verbs_by_aspect[k]:>6}   gap slots {gap_by_aspect[k]:>7}')
print('   top gap slots:', gap_slots.most_common(6))
print()
print('== +p vs attested past passive participle ==')
for (hasp, hasppp), n in sorted(ppp.items()):
    print(f'   code {"has" if hasp else "no "} +p, PPP {"yes" if hasppp else "no "}: {n:>6}')
print()
print('== present-stem mutations ==')
for k, v in muts.most_common(20):
    print(f'   {k:<14} {v:>5}   {", ".join(mut_ex[k])}')
print()
print('== class 1 labial-final stems ==', dict(lab_no_epen))
print()
print('== noun stem classes ==', noun_stem.most_common())
print('== noun accents ==', noun_accent.most_common())
print('== verb accent letters ==', verb_accent.most_common())
print('== adjective kinds ==', adj_kind.most_common())
print()
print(f'candidates: verbs {len(verbs)}  nouns {len(nouns)}  adjs {len(adjs)}')

# ---- write the full code corpus ----
with io.open('class-codes-full.txt', 'w', encoding='utf-8') as f:
    for c in sorted(codes): f.write(c + '\n')
print(f'wrote class-codes-full.txt ({len(codes)} codes)')

# ---- random held-out sample: fixed seed, no hand-picking, no class targeting ----
random.seed(20260725)
def emit(name, pool, source, n):
    pick = random.sample(pool, min(n, len(pool)))
    rows = meta = 0
    with io.open(f'{name}.tsv', 'w', encoding='utf-8') as fo, \
         io.open(f'{name}_meta.tsv', 'w', encoding='utf-8') as fm_:
        for w, d, cls in pick:
            ht = (d.get('head_templates') or [{}])[0]
            it = (d.get('inflection_templates') or [{}])[0]
            exp = ht.get('expansion', '')
            extra = []
            if d.get('pos') == 'verb':
                extra.append('aspect=' + it.get('args', {}).get('1', '?'))
                for k, v in it.get('args', {}).items():
                    if k not in ('1','2','3'): extra.append(f'arg:{k}={v}')
            else:
                g = re.search(r'\)\s+(m|f|n)\b', exp)
                if g: extra.append({'m':'masculine','f':'feminine','n':'neuter'}[g.group(1)])
                if 'anim' in exp: extra.append('inanimate' if 'inan' in exp else 'animate')
                if ht.get('args', {}).get('2') == '*': extra.append('reducible')
            extra += ['cls:' + c for c in classes_of(d)]
            fm_.write(f'{w}\t{d.get("pos")}\t{cls}\t{";".join(extra)}\trandom\n'); meta += 1
            for slot, form in sorted(forms_map(d, source).items()):
                if form and form != '-':
                    fo.write(f'{w}\t{d.get("pos")}\t{cls}\t{slot}\t{form}\n'); rows += 1
    print(f'wrote {name}: {meta} lemmas, {rows} rows')

emit('random_nouns', nouns, 'declension', 150)
emit('random_verbs', verbs, 'conjugation', 150)
emit('random_adjs',  adjs,  'declension', 100)
