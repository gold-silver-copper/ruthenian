"""Measured ending inventories per language, from FULL scans (DIRECTION.md law 3).

Produces the ending tables in docs/COMPARATIVE_GRAMMAR.md. Inputs are complete
per-language record sets, each from one pass over the whole dump:

    LC_ALL=C grep -F '"lang_code": "ru"'      raw-wiktextract-data.jsonl > ru_all.jsonl
    LC_ALL=C grep -F -e '"lang_code": "uk"' -e '"lang_code": "be"' \
                    -e '"lang_code": "sa"' -e '"lang_code": "cu"' \
                    raw-wiktextract-data.jsonl > others.jsonl
    LC_ALL=C grep -F '"lang_code": "ine-pro"' raw-wiktextract-data.jsonl > pie.jsonl


For each lemma: stem = longest common prefix over all its attested forms
(diacritics stripped); ending = form minus stem. Tally by
(language, pos, gender/class, case, number).
"""
import json, sys, collections, unicodedata

CASES = ['nominative','vocative','accusative','genitive','ablative','dative',
         'instrumental','locative','prepositional','partitive']
NUMS = ['singular','dual','plural']
GEND = ['masculine','feminine','neuter']

def strip_marks(s):
    # Remove combining marks (Russian/Ukrainian stress, Sanskrit accents) but keep
    # base letters, so stems align across an accented paradigm.
    return ''.join(c for c in unicodedata.normalize('NFD', s)
                   if not unicodedata.combining(c))

def lcp(strings):
    if not strings: return ''
    p = strings[0]
    for s in strings[1:]:
        while not s.startswith(p):
            p = p[:-1]
            if not p: return ''
    return p

QUAL = {'dated','obsolete','archaic','colloquial','rare','literary','poetic',
        'proscribed','nonstandard','informal','regional','dialectal'}

def cells(d, source):
    """slot-tuple -> form, for clean single-word forms."""
    out = {}
    for f in d.get('forms', []):
        t = set(f.get('tags', []))
        if t & QUAL or 'romanization' in t or 'canonical' in t: continue
        if f.get('source') and f['source'] != source: continue
        form = f.get('form')
        if not form or form in ('-', '—') or ' ' in form: continue
        c = next((x for x in CASES if x in t), None)
        n = next((x for x in NUMS if x in t), None)
        if c and n:
            out.setdefault((c, n), strip_marks(form))
    return out

noun_end = collections.defaultdict(collections.Counter)   # (lang,gender,case,num) -> ending
verb_end = collections.defaultdict(collections.Counter)   # (lang,person,num,tense) -> ending
syncret  = collections.defaultdict(collections.Counter)   # (lang,num) -> 'abl==dat' etc.
dual_end = collections.defaultdict(collections.Counter)
voc_end  = collections.defaultdict(collections.Counter)

PERS = ['first-person','second-person','third-person']
TENSE = ['present','aorist','imperfect','future','past','perfect']

def run(path, langs):
    for line in open(path, encoding='utf-8'):
        try: d = json.loads(line)
        except Exception: continue
        lc = d.get('lang_code')
        if lc not in langs: continue
        pos = d.get('pos')

        if pos == 'noun':
            fm = cells(d, 'declension')
            if len(fm) < 4: continue
            stem = lcp(list(fm.values()))
            g = None
            for f in d.get('forms', []):
                t = set(f.get('tags', []))
                if 'canonical' in t:
                    g = next((x for x in GEND if x in t), None)
            if g is None:
                exp = (d.get('head_templates') or [{}])[0].get('expansion','')
                import re
                m = re.search(r'\)\s+(m|f|n)\b', exp)
                g = {'m':'masculine','f':'feminine','n':'neuter'}.get(m.group(1)) if m else 'unknown'
            for (c, n), form in fm.items():
                if form.startswith(stem):
                    noun_end[(lc, g, c, n)][form[len(stem):] or '∅'] += 1
            # case syncretism within a number
            for n in NUMS:
                have = {c: fm[(c, n)] for c in CASES if (c, n) in fm}
                for a, b in (('ablative','dative'), ('ablative','genitive'),
                             ('genitive','locative'), ('dative','instrumental'),
                             ('nominative','accusative'), ('nominative','vocative')):
                    if a in have and b in have:
                        syncret[(lc, n)][f'{a}={b}' if have[a] == have[b] else f'{a}≠{b}'] += 1
            if ('nominative','dual') in fm:
                for c in CASES:
                    if (c,'dual') in fm and fm[(c,'dual')].startswith(stem):
                        dual_end[(lc, c)][fm[(c,'dual')][len(stem):] or '∅'] += 1
            if ('vocative','singular') in fm and fm[('vocative','singular')].startswith(stem):
                voc_end[(lc, g)][fm[('vocative','singular')][len(stem):] or '∅'] += 1

        elif pos == 'verb':
            fm = {}
            for f in d.get('forms', []):
                t = set(f.get('tags', []))
                if t & QUAL or 'romanization' in t: continue
                form = f.get('form')
                if not form or form in ('-','—') or ' ' in form: continue
                p = next((x for x in PERS if x in t), None)
                n = next((x for x in NUMS if x in t), None)
                te = next((x for x in TENSE if x in t), None)
                if p and n and te:
                    fm.setdefault((p, n, te), strip_marks(form))
            if len(fm) < 4: continue
            stem = lcp(list(fm.values()))
            for (p, n, te), form in fm.items():
                if form.startswith(stem):
                    verb_end[(lc, p, n, te)][form[len(stem):] or '∅'] += 1

run('ru_all.jsonl', {'ru'})
run('others.jsonl', {'uk','be','cu','sa'})
run('pie.jsonl', {'ine-pro'})

def top(counter, k=3):
    tot = sum(counter.values())
    return ', '.join(f'{e} ({100*n//max(tot,1)}%)' for e, n in counter.most_common(k))

print('=' * 78)
print('NOUN ENDINGS — most common, by language / gender / case / number')
print('=' * 78)
for lc in ['ine-pro','sa','cu','ru','uk','be']:
    print(f'\n### {lc}')
    for g in GEND:
        rows = [(c, n) for n in NUMS for c in CASES if noun_end[(lc,g,c,n)]]
        if not rows: continue
        print(f'  -- {g} --')
        for n in NUMS:
            for c in CASES:
                k = (lc, g, c, n)
                if noun_end[k] and sum(noun_end[k].values()) >= 5:
                    print(f'    {c:<14} {n:<9} {top(noun_end[k])}')

print()
print('=' * 78)
print('CASE SYNCRETISM (how often two cases are identical)')
print('=' * 78)
for lc in ['ine-pro','sa','cu','ru','uk','be']:
    for n in NUMS:
        c = syncret[(lc, n)]
        if not c: continue
        pairs = {}
        for k, v in c.items():
            base = k.replace('≠','=').split('=')[0] + '=' + k.split('=' if '=' in k else '≠')[-1]
        items = collections.defaultdict(lambda: [0,0])
        for k, v in c.items():
            eq = '=' in k
            key = k.replace('≠','=')
            items[key][0 if eq else 1] += v
        out = []
        for key, (same, diff) in sorted(items.items()):
            tot = same + diff
            if tot >= 20: out.append(f'{key} {100*same//tot}%')
        if out: print(f'  {lc:<8} {n:<9} ' + '  '.join(out))

print()
print('=' * 78)
print('DUAL ENDINGS')
print('=' * 78)
for lc in ['ine-pro','sa','cu']:
    for c in CASES:
        if dual_end[(lc,c)] and sum(dual_end[(lc,c)].values()) >= 5:
            print(f'  {lc:<8} {c:<14} {top(dual_end[(lc,c)], 4)}')

print()
print('=' * 78)
print('VOCATIVE SINGULAR ENDINGS')
print('=' * 78)
for lc in ['ine-pro','sa','cu','uk','ru']:
    for g in GEND + ['unknown']:
        if voc_end[(lc,g)] and sum(voc_end[(lc,g)].values()) >= 5:
            print(f'  {lc:<8} {g:<10} {top(voc_end[(lc,g)], 4)}')

print()
print('=' * 78)
print('VERB PERSONAL ENDINGS — present/aorist')
print('=' * 78)
for lc in ['ine-pro','sa','cu','ru','uk','be']:
    print(f'\n### {lc}')
    for te in ['present','aorist','imperfect','future','past']:
        for n in NUMS:
            for p in PERS:
                k = (lc, p, n, te)
                if verb_end[k] and sum(verb_end[k].values()) >= 5:
                    print(f'  {te:<10} {p:<14} {n:<9} {top(verb_end[k])}')
