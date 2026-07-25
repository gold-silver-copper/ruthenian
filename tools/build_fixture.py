"""Build the targeted paradigm fixture from a FULL dump scan.

Reads `ru_all.jsonl` on stdin — the complete set of Russian records,
produced by one pass over the whole dump:

    LC_ALL=C grep -F '\"lang_code\": \"ru\"' raw-wiktextract-data.jsonl > ru_all.jsonl
    python3 tools/build_fixture.py < ru_all.jsonl

Never a window, a `dd` slice or a `head`. See INVARIANTS.md I1.
"""

import json, sys, collections, re, unicodedata

STRESS = '́'
def bare(s): return s.replace(STRESS, '')

WANT_VERB_CLASSES = ['1a','1a+p','2a','2a+p','3a','3a+p','4a','4a+p','4b','4b+p','4c','4c+p',
                     '5a','5b','5c','6b','6c','6c+p','7b','8b/b','11b','12a','13b','14b','15a','16b/c',
                     'irreg','-']
NAMED = set('победить идти быть есть хотеть дать бежать мать дочь время путь ребёнок человек '
            'любить писать нести мочь брать жить ждать стать взять сказать говорить читать '
            'делать работать купить платить просить возить водить ходить носить '
            'ночь дверь имя знамя год город друг брат стул дерево окно земля вода стол книга '
            'ножницы кофе метро пальто сестра сосед '
            'любой синий хороший большой русский молодой сухой'.split())

verbs = collections.defaultdict(list)   # class -> [(lemma, rec)]
muts  = collections.defaultdict(list)   # mutation -> [(lemma, rec)]
nouns = collections.defaultdict(list)   # (stem, accent) -> [...]
adjs  = collections.defaultdict(list)
named = {}

QUAL = {'dated','obsolete','archaic','colloquial','rare','irregular','literary','poetic','proscribed','nonstandard','informal','regional'}
def slot_of(f):
    tags = [t for t in f.get('tags', []) if t not in ('table-tags','class','inflection-template','romanization','canonical')]
    if any(t in QUAL for t in tags): return None
    return ' '.join(sorted(tags))

def forms_map(d):
    m = {}
    for f in d.get('forms', []):
        if 'romanization' in f.get('tags', []): continue
        if f.get('source') not in ('conjugation','declension') and 'canonical' not in f.get('tags',[]): continue
        s = slot_of(f)
        if s and s not in m:
            m[s] = f.get('form')
    return m

def classes_of(d):
    return [f['form'] for f in d.get('forms', []) if 'class' in f.get('tags', [])]

def detect_mutation(lemma, fm):
    one = fm.get('first-person present singular') or fm.get('first-person future singular')
    if not one or one == '-' or ' ' in one: return None
    inf, o = bare(lemma), bare(one)
    if len(inf) < 4 or inf[-3:] not in ('ить','ать','еть','ять','оть','уть'): return None
    stem = inf[:-3]
    if o.startswith(stem + 'л'): return f'{stem[-1]}->{stem[-1]}л'
    if o.startswith(stem): return None
    i = 0
    while i < min(len(stem), len(o)) and stem[i] == o[i]: i += 1
    a, b = stem[i:], re.sub('(у|ю)$', '', o[i:])
    if a and len(a) <= 2 and len(b) <= 3: return f'{a}->{b}'
    return None

for line in sys.stdin:
    if '"lang_code": "ru"' not in line: continue
    try: d = json.loads(line)
    except Exception: continue
    if d.get('lang_code') != 'ru': continue
    pos, word = d.get('pos'), d.get('word') or ''
    if not word or ' ' in word or word.startswith('-'): continue
    ht = (d.get('head_templates') or [{}])[0]
    it = (d.get('inflection_templates') or [{}])[0]
    fm = forms_map(d)
    if len(fm) < 4: continue

    if pos == 'verb' and it.get('name') == 'ru-conj':
        cls = it.get('args', {}).get('2', '')
        rec = (word, d, cls)
        if len(verbs[cls]) < 2: verbs[cls].append(rec)
        m = detect_mutation(word, fm)
        if m and len(muts[m]) < 2: muts[m].append(rec)
        if bare(word) in NAMED and bare(word) not in named: named[bare(word)] = rec
    elif pos == 'noun' and ht.get('name','').startswith('ru-noun'):
        cl = classes_of(d)
        stem = next((c for c in cl if 'stem' in c), '?')
        acc  = next((c for c in cl if c.startswith('accent')), '?')
        rec = (word, d, f'{stem}|{acc}')
        if len(nouns[(stem,acc)]) < 1: nouns[(stem,acc)].append(rec)
        if bare(word) in NAMED and bare(word) not in named: named[bare(word)] = rec
    elif pos == 'adj' and ht.get('name') == 'ru-adj':
        cl = classes_of(d)
        key = cl[0] if cl else '?'
        rec = (word, d, key)
        if len(adjs[key]) < 1: adjs[key].append(rec)
        if bare(word) in NAMED and bare(word) not in named: named[bare(word)] = rec

chosen, seen = [], set()
def take(rec, why):
    word, d, cls = rec
    key = (word, d.get('pos'))
    if key in seen: return
    seen.add(key); chosen.append((rec, why))

for c in WANT_VERB_CLASSES:
    for r in verbs.get(c, []): take(r, f'verb class {c}')
for m, rs in sorted(muts.items()):
    for r in rs[:1]: take(r, f'mutation {m}')
for k, rs in sorted(nouns.items()):
    for r in rs: take(r, f'noun {k[0]} {k[1]}')
for k, rs in sorted(adjs.items()):
    for r in rs: take(r, f'adj class {k}')
for w, r in sorted(named.items()):
    take(r, 'named hard case')

out = open('fixture.tsv', 'w', encoding='utf-8')
meta = open('fixture_meta.tsv', 'w', encoding='utf-8')
rows = 0
for (word, d, cls), why in chosen:
    pos = d.get('pos')
    ht = (d.get('head_templates') or [{}])[0]
    it = (d.get('inflection_templates') or [{}])[0]
    fm = forms_map(d)
    canon = next((f['form'] for f in d.get('forms', []) if 'canonical' in f.get('tags', [])), word)
    tags = next((f.get('tags', []) for f in d.get('forms', []) if 'canonical' in f.get('tags', [])), [])
    extra = []
    if pos == 'verb':
        extra.append('aspect=' + it.get('args', {}).get('1', '?'))
        for k, v in it.get('args', {}).items():
            if k not in ('1','2','3'): extra.append(f'arg:{k}={v}')
        for c in classes_of(d): extra.append('cls:' + c)
    else:
        # Gender and animacy are lexicon facts, not rule outputs. Read them from
        # the headword expansion ("• (dvérʹ) f inan"), which always carries them,
        # rather than from the canonical form's tags, which often do not.
        exp = ht.get('expansion','')
        seen_g = [t for t in tags if t in ('masculine','feminine','neuter')]
        if not seen_g:
            m2 = re.search(r'\)\s+(m|f|n)\b', exp)
            if m2: seen_g = [{'m':'masculine','f':'feminine','n':'neuter'}[m2.group(1)]]
        extra.extend(seen_g)
        if 'anim' in exp:
            extra.append('inanimate' if 'inan' in exp else 'animate')
        else:
            extra.extend(t for t in tags if t in ('animate','inanimate'))
        for c in classes_of(d): extra.append('cls:' + c)
        if ht.get('args', {}).get('2') == '*': extra.append('reducible')
    meta.write(f"{word}\t{pos}\t{cls}\t{';'.join(extra)}\t{why}\n")
    for slot, form in sorted(fm.items()):
        if not slot: continue
        out.write(f"{word}\t{pos}\t{cls}\t{slot}\t{form}\n")
        rows += 1
out.close(); meta.close()
print(f'lemmas: {len(chosen)}  form rows: {rows}')
print('verb classes covered:', len({c for (w,d,c),_ in chosen if d.get("pos")=="verb"}))
print('mutations covered:', len(muts))
