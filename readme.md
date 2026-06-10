# dictionaries

Collection of normalized [hunspell][] dictionaries, used by
[Codebook][codebook].

This is a fork of [`wooorm/dictionaries`][upstream] that replaces the original
JavaScript/npm pipeline with a Rust tool, strips the output down to the raw
hunspell files (`index.aff`, `index.dic`, `license`), and validates every
dictionary with [spellbook][] — the spell checker Codebook uses internally —
so anything checked in here is known to load.

## Layout

Each dictionary lives at `dictionaries/<BCP-47 code>/`:

*   `index.aff` — affix file (UTF-8, `SET UTF-8`)
*   `index.dic` — word list (UTF-8)
*   `license` — the upstream license file, when the source ships one

## Building

The pipeline is one Rust binary with subcommands:

```sh
cargo run --release -- crawl      # download + extract upstream sources (archive/, source/)
cargo run --release -- build      # make/configure for sources that build their files
cargo run --release -- generate   # decode, normalize, patch, write dictionaries/
cargo run --release -- readme     # regenerate the tables in this readme
cargo run --release -- validate   # parse every dictionary with spellbook
cargo run --release -- all        # all of the above, in order
```

Every subcommand accepts a repeatable `--only <code>` filter, e.g.
`cargo run -- all --only uk`.
A `Makefile` wraps the common invocations — run `make help` for the list.

Notes:

*   `validate` works offline against the checked-in dictionaries — no crawl
    needed. Run it after any change.
*   `crawl` caches downloads in `archive/` and extractions in `source/`
    (both gitignored). Delete an entry to re-fetch it.
*   `build` shells out to `make` for six sources that don't ship ready-made
    hunspell files (`de`, `el`, `gd`, `nds`, `rw`, `he`). Host
    prerequisites: `make` and `perl`, plus the `ispell` and `hunspell` CLIs
    for `de` (`brew install ispell hunspell`), and a C toolchain for `he`.
*   `patch --only <code>` applies registered patches to already-generated
    dictionaries in place, for adding a patch without re-crawling. Patches
    fail loudly when their target text is absent, so they can't double-apply.
*   Upstream URLs, paths, and encodings live in `src/table.rs`, ported from
    upstream's `script/crawl.sh` with the original provenance comments.

## Fork modifications

Some dictionaries are modified after normalization, before writing. The
patches live in `src/patches.rs`; each asserts the exact upstream text it
expects and fails the build when upstream changes, so this list (generated
from the patch registry) is always current:

<!--patches start-->

*   **`uk`**: remove ICONV rules that mapped every Latin letter to the digit 0, which made hunspell silently accept all Latin-script words as numbers
*   **`da`**: escape `/` in three slash-containing entries (quoting is not valid dic syntax) and fix a corrupted FedEx/Fedkrog line
*   **`el-polyton`**: replace a tab with a space inside a REP rule (hunspell treats the tab as a field separator, breaking the rule)
*   **`br`**: fix three entries flagged `m01` — with FLAG long that is one-and-a-half flags; the defined flag `m0` is meant
*   **`gl`**: fix a corrupted numeric flag `2iñer30` (stray text spliced into `230`, the present-tense suffix flag) on the entry for `tumbar`
*   **`hy`**: fix the `SFX VD` block header declaring 171 rules when 172 follow
*   **`ia`**: remove a doubled closing bracket (`[oiyu]]`) from seven PFX conditions
*   **`la`**: fix two affix rules misspelled `SFK` instead of `SFX`, which also broke the surrounding block's declared rule count
*   **`mn`**: remove the COMPOUNDRULE block — its `[a0,a1,...]` alternation syntax is not valid hunspell and no parser accepts it
*   **`ne`**: strip a stray `X` from 171 numeric continuation flags (`17X` → `17`); the X-less flags are the ones actually defined, matching hunspell's lenient numeric parsing; also fix three corrupted entries (a slash inside a word, a stray `I` in a flag list, and two lines merged into one)
*   **`tr`**: renumber affix flag `0` to `9999` — hunspell numeric flags are defined as 1–65000 and flag 0 is rejected by spellbook

<!--patches end-->

## List of dictionaries

> 👉 **Note**: preferred BCP-47 codes are used (according to Unicode CLDR).
> To illustrate,
> as American English and Brazilian Portuguese are the most common types of
> English and Portuguese respectively,
> they get the codes `en` and `pt`.

<!--support start-->

In total 92 dictionaries are provided.

| Code | Language | License | Source |
| - | - | - | - |
| [`bg`](dictionaries/bg) | Bulgarian | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/bg/license) | [bgoffice.sourceforge.net](http://bgoffice.sourceforge.net) |
| [`br`](dictionaries/br) | Breton | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/br/license) | [Drouizig/hunspell-br](https://github.com/Drouizig/hunspell-br) |
| [`ca`](dictionaries/ca) | Catalan | [(GPL-2.0 OR LGPL-2.1)](dictionaries/ca/license) | [Softcatala/catalan-dict-tools](https://github.com/Softcatala/catalan-dict-tools) |
| [`ca-valencia`](dictionaries/ca-valencia) | Catalan (Valencia) | [(GPL-2.0 OR LGPL-2.1)](dictionaries/ca-valencia/license) | [Softcatala/catalan-dict-tools](https://github.com/Softcatala/catalan-dict-tools) |
| [`cs`](dictionaries/cs) | Czech | [GPL-2.0](dictionaries/cs/license) | [translatoblog.cz](http://www.translatoblog.cz/hunspell/) |
| [`cy`](dictionaries/cy) | Welsh | [LGPL-3.0](dictionaries/cy/license) | [techiaith/hunspell-cy](https://github.com/techiaith/hunspell-cy) |
| [`da`](dictionaries/da) | Danish | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/da/license) | [stavekontrolden.dk](https://stavekontrolden.dk) |
| [`de`](dictionaries/de) | German | [(GPL-2.0 OR GPL-3.0)](dictionaries/de/license) | [j3e.de](https://www.j3e.de/ispell/igerman98/index_en.html) |
| [`de-AT`](dictionaries/de-AT) | German (Austria) | [(GPL-2.0 OR GPL-3.0)](dictionaries/de-AT/license) | [j3e.de](https://www.j3e.de/ispell/igerman98/index_en.html) |
| [`de-CH`](dictionaries/de-CH) | German (Switzerland) | [(GPL-2.0 OR GPL-3.0)](dictionaries/de-CH/license) | [j3e.de](https://www.j3e.de/ispell/igerman98/index_en.html) |
| [`el`](dictionaries/el) | Greek | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/el/license) | [stevestavropoulos/elspell](https://github.com/stevestavropoulos/elspell) |
| [`el-polyton`](dictionaries/el-polyton) | Greek (Polyton) | [GPL-3.0](dictionaries/el-polyton/license) | [thepolytonicproject.gr](https://thepolytonicproject.gr/spell) |
| [`en`](dictionaries/en) | English | [(MIT AND BSD)](dictionaries/en/license) | [wordlist.aspell.net](http://wordlist.aspell.net/dicts/) |
| [`en-AU`](dictionaries/en-AU) | English (Australia) | [(MIT AND BSD)](dictionaries/en-AU/license) | [wordlist.aspell.net](http://wordlist.aspell.net/dicts/) |
| [`en-CA`](dictionaries/en-CA) | English (Canada) | [(MIT AND BSD)](dictionaries/en-CA/license) | [wordlist.aspell.net](http://wordlist.aspell.net/dicts/) |
| [`en-GB`](dictionaries/en-GB) | English (United Kingdom) | [(MIT AND BSD)](dictionaries/en-GB/license) | [wordlist.aspell.net](http://wordlist.aspell.net/dicts/) |
| [`en-ZA`](dictionaries/en-ZA) | English (South Africa) | [LGPL-2.1](dictionaries/en-ZA/license) | [extensions.openoffice.org](https://extensions.openoffice.org/en/project/english-dictionaries-apache-openoffice) |
| [`eo`](dictionaries/eo) | Esperanto | [GPL-2.0](dictionaries/eo/license) | [esperantilo.org](http://www.esperantilo.org/index_en.html) |
| [`es`](dictionaries/es) | Spanish | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-AR`](dictionaries/es-AR) | Spanish (Argentina) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-AR/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-BO`](dictionaries/es-BO) | Spanish (Bolivia) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-BO/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-CL`](dictionaries/es-CL) | Spanish (Chile) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-CL/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-CO`](dictionaries/es-CO) | Spanish (Colombia) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-CO/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-CR`](dictionaries/es-CR) | Spanish (Costa Rica) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-CR/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-CU`](dictionaries/es-CU) | Spanish (Cuba) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-CU/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-DO`](dictionaries/es-DO) | Spanish (Dominican Republic) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-DO/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-EC`](dictionaries/es-EC) | Spanish (Ecuador) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-EC/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-GT`](dictionaries/es-GT) | Spanish (Guatemala) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-GT/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-HN`](dictionaries/es-HN) | Spanish (Honduras) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-HN/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-MX`](dictionaries/es-MX) | Spanish (Mexico) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-MX/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-NI`](dictionaries/es-NI) | Spanish (Nicaragua) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-NI/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-PA`](dictionaries/es-PA) | Spanish (Panama) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-PA/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-PE`](dictionaries/es-PE) | Spanish (Peru) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-PE/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-PH`](dictionaries/es-PH) | Spanish (Philippines) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-PH/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-PR`](dictionaries/es-PR) | Spanish (Puerto Rico) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-PR/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-PY`](dictionaries/es-PY) | Spanish (Paraguay) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-PY/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-SV`](dictionaries/es-SV) | Spanish (El Salvador) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-SV/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-US`](dictionaries/es-US) | Spanish (United States of America) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-US/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-UY`](dictionaries/es-UY) | Spanish (Uruguay) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-UY/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`es-VE`](dictionaries/es-VE) | Spanish (Venezuela) | [(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)](dictionaries/es-VE/license) | [sbosio/rla-es](https://github.com/sbosio/rla-es) |
| [`et`](dictionaries/et) | Estonian | LGPL-2.1 | [meso.ee](http://www.meso.ee/~jjpp/speller) |
| [`eu`](dictionaries/eu) | Basque | GPL-2.0 | [xuxen.eus](http://xuxen.eus/eu/home) |
| [`fa`](dictionaries/fa) | Persian | [Apache-2.0](dictionaries/fa/license) | [b00f/lilak](https://github.com/b00f/lilak) |
| [`fo`](dictionaries/fo) | Faroese | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/fo/license) | (frozen — upstream gone) |
| [`fr`](dictionaries/fr) | French | [MPL-2.0](dictionaries/fr/license) | [grammalecte.net](https://grammalecte.net) |
| [`fur`](dictionaries/fur) | Friulian | [GPL-2.0](dictionaries/fur/license) | (frozen — upstream gone) |
| [`fy`](dictionaries/fy) | Western Frisian | [GPL-3.0](dictionaries/fy/license) | [PanderMusubi/frisian](https://github.com/PanderMusubi/frisian) |
| [`ga`](dictionaries/ga) | Irish | [GPL-2.0](dictionaries/ga/license) | [kscanne/gaelspell](https://github.com/kscanne/gaelspell) |
| [`gd`](dictionaries/gd) | Scottish Gaelic | [GPL-3.0](dictionaries/gd/license) | [kscanne/hunspell-gd](https://github.com/kscanne/hunspell-gd) |
| [`gl`](dictionaries/gl) | Galician | [GPL-3.0](dictionaries/gl/license) | [meixome/hunspell-gl](https://github.com/meixome/hunspell-gl) |
| [`he`](dictionaries/he) | Hebrew | [AGPL-3.0](dictionaries/he/license) | [hspell.ivrix.org.il](http://hspell.ivrix.org.il) |
| [`hr`](dictionaries/hr) | Croatian | [(LGPL-2.1 OR SISSL)](dictionaries/hr/license) | [krunose/hunspell-hr](https://github.com/krunose/hunspell-hr) |
| [`hu`](dictionaries/hu) | Hungarian | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/hu/license) | [laszlonemeth/magyarispell](https://github.com/laszlonemeth/magyarispell) |
| [`hy`](dictionaries/hy) | Armenian | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/hy/license) | (frozen — upstream gone) |
| [`hyw`](dictionaries/hyw) | Western Armenian | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/hyw/license) | (frozen — upstream gone) |
| [`ia`](dictionaries/ia) | Interlingua | [GPL-3.0](dictionaries/ia/license) | [addons.thunderbird.net](https://addons.thunderbird.net/en-US/thunderbird/addon/dict-ia/) |
| [`ie`](dictionaries/ie) | Interlingue | [Apache-2.0](dictionaries/ie/license) | [Carmina16/hunspell-ie](https://github.com/Carmina16/hunspell-ie) |
| [`is`](dictionaries/is) | Icelandic | [CC-BY-SA-3.0](dictionaries/is/license) | [LibreOffice/dictionaries](https://github.com/LibreOffice/dictionaries) |
| [`it`](dictionaries/it) | Italian | [GPL-3.0](dictionaries/it/license) | (frozen — upstream gone) |
| [`ka`](dictionaries/ka) | Georgian | [MIT](dictionaries/ka/license) | [gamag/ka_GE.spell](https://github.com/gamag/ka_GE.spell) |
| [`ko`](dictionaries/ko) | Korean | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/ko/license) | [spellcheck-ko/hunspell-dict-ko](https://github.com/spellcheck-ko/hunspell-dict-ko) |
| [`la`](dictionaries/la) | Latin | [GPL-2.0](dictionaries/la/license) | [extensions.openoffice.org](https://extensions.openoffice.org/project/dict-la) |
| [`lb`](dictionaries/lb) | Luxembourgish | [EUPL-1.1](dictionaries/lb/license) | [spellchecker-lu/dictionary-lb-lu](https://github.com/spellchecker-lu/dictionary-lb-lu) |
| [`lt`](dictionaries/lt) | Lithuanian | [BSD-3-Clause](dictionaries/lt/license) | [ispell-lt/ispell-lt](https://github.com/ispell-lt/ispell-lt) |
| [`ltg`](dictionaries/ltg) | Latgalian | [LGPL-2.1](dictionaries/ltg/license) | [dict.dv.lv](http://dict.dv.lv/home.php?prj=la) |
| [`lv`](dictionaries/lv) | Latvian | [LGPL-2.1](dictionaries/lv/license) | [dict.dv.lv](http://dict.dv.lv/home.php?prj=lv) |
| [`mk`](dictionaries/mk) | Macedonian | [GPL-3.0](dictionaries/mk/license) | (frozen — upstream gone) |
| [`mn`](dictionaries/mn) | Mongolian | [LPPL-1.3c](dictionaries/mn/license) | [bataak/dict-mn](https://github.com/bataak/dict-mn) |
| [`nb`](dictionaries/nb) | Norwegian Bokmål | [GPL-2.0](dictionaries/nb/license) | [no.speling.org](http://no.speling.org) |
| [`nds`](dictionaries/nds) | Low German | [GPL-3.0](dictionaries/nds/license) | [tdf/dict_nds](https://github.com/tdf/dict_nds) |
| [`ne`](dictionaries/ne) | Nepali | [LGPL-2.1](dictionaries/ne/license) | [ltk.org.np](http://ltk.org.np) |
| [`nl`](dictionaries/nl) | Dutch | [(BSD-3-Clause OR CC-BY-3.0)](dictionaries/nl/license) | [OpenTaal/opentaal-hunspell](https://github.com/OpenTaal/opentaal-hunspell) |
| [`nn`](dictionaries/nn) | Norwegian Nynorsk | [GPL-2.0](dictionaries/nn/license) | [no.speling.org](http://no.speling.org) |
| [`oc`](dictionaries/oc) | Occitan | [GPL-2.0](dictionaries/oc/license) | [gl:taissou/hunspell-files-for-occitan-lengadocian](https://gitlab.com/taissou/hunspell-files-for-occitan-lengadocian) |
| [`pl`](dictionaries/pl) | Polish | [(GPL-3.0 OR LGPL-3.0 OR MPL-2.0)](dictionaries/pl/license) | [extensions.openoffice.org](http://extensions.openoffice.org/en/project/polish-dictionary-pack) |
| [`pt`](dictionaries/pt) | Portuguese | [(LGPL-3.0 OR MPL-2.0)](dictionaries/pt/license) | [LibreOffice/dictionaries](https://github.com/LibreOffice/dictionaries) |
| [`pt-PT`](dictionaries/pt-PT) | Portuguese (Portugal) | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/pt-PT/license) | [natura.di.uminho.pt](https://natura.di.uminho.pt) |
| [`ro`](dictionaries/ro) | Romanian | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/ro/license) | [rospell.wordpress.com](https://rospell.wordpress.com) |
| [`ru`](dictionaries/ru) | Russian | [BSD-3-Clause](dictionaries/ru/license) | [LibreOffice/dictionaries](https://github.com/LibreOffice/dictionaries) |
| [`rw`](dictionaries/rw) | Kinyarwanda | [GPL-3.0](dictionaries/rw/license) | [kscanne/hunspell-rw](https://github.com/kscanne/hunspell-rw) |
| [`sk`](dictionaries/sk) | Slovak | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)](dictionaries/sk/license) | [sk-spell.sk.cx](http://www.sk-spell.sk.cx) |
| [`sl`](dictionaries/sl) | Slovenian | [(GPL-3.0 OR LGPL-2.1)](dictionaries/sl/license) | [extensions.libreoffice.org](https://extensions.libreoffice.org/extensions/slovenian-dictionary-pack/) |
| [`sr`](dictionaries/sr) | Serbian | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1 OR CC-BY-SA-3.0)](dictionaries/sr/license) | [grakic/hunspell-sr](https://github.com/grakic/hunspell-sr) |
| [`sr-Latn`](dictionaries/sr-Latn) | Serbian (Latin script) | [(GPL-2.0 OR LGPL-2.1 OR MPL-1.1 OR CC-BY-SA-3.0)](dictionaries/sr-Latn/license) | [grakic/hunspell-sr](https://github.com/grakic/hunspell-sr) |
| [`sv`](dictionaries/sv) | Swedish | [LGPL-3.0](dictionaries/sv/license) | [extensions.libreoffice.org](https://extensions.libreoffice.org/extensions/swedish-spelling-dictionary-den-stora-svenska-ordlistan) |
| [`sv-FI`](dictionaries/sv-FI) | Swedish (Finland) | [LGPL-3.0](dictionaries/sv-FI/license) | [extensions.libreoffice.org](https://extensions.libreoffice.org/extensions/swedish-spelling-dictionary-den-stora-svenska-ordlistan) |
| [`tk`](dictionaries/tk) | Turkmen | [Apache-2.0](dictionaries/tk/license) | [nazartm/turkmen-spell-check-dictionary](https://github.com/nazartm/turkmen-spell-check-dictionary) |
| [`tlh`](dictionaries/tlh) | Klingon | [Apache-2.0](dictionaries/tlh/license) | [PanderMusubi/klingon](https://github.com/PanderMusubi/klingon) |
| [`tlh-Latn`](dictionaries/tlh-Latn) | Klingon (Latin script) | [Apache-2.0](dictionaries/tlh-Latn/license) | [PanderMusubi/klingon](https://github.com/PanderMusubi/klingon) |
| [`tr`](dictionaries/tr) | Turkish | [MIT](dictionaries/tr/license) | [extensions.openoffice.org](http://extensions.openoffice.org/en/project/turkish-spellcheck-dictionary) |
| [`uk`](dictionaries/uk) | Ukrainian | [GPL-3.0](dictionaries/uk/license) | [brown-uk/dict_uk](https://github.com/brown-uk/dict_uk) |
| [`vi`](dictionaries/vi) | Vietnamese | [GPL-2.0](dictionaries/vi/license) | [1ec5/hunspell-vi](https://github.com/1ec5/hunspell-vi) |

<!--support end-->

## License

The build tool is [MIT][file-license] licensed (© Titus Wormer for the
original pipeline this is derived from).
Each dictionary keeps its own upstream license — see the table above and the
`license` file in each dictionary directory.

[codebook]: https://github.com/blopker/codebook

[file-license]: license

[hunspell]: https://hunspell.github.io

[spellbook]: https://github.com/helix-editor/spellbook

[upstream]: https://github.com/wooorm/dictionaries
