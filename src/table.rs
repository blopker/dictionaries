//! Declarative tables describing every upstream source and every generated
//! dictionary. Ported from the upstream `script/crawl.sh`.

/// An upstream source: one downloadable archive (or set of files) that one or
/// more dictionaries are generated from.
pub struct Source {
    /// Directory name under `source/`, e.g. `"danish"`.
    pub name: &'static str,
    /// Human-facing page describing the source (provenance, shown in readme).
    pub page: &'static str,
    /// How to obtain the source files.
    pub fetch: Fetch,
    /// Extra files fetched into `source/<name>/` after extraction
    /// (e.g. license files not shipped in the archive).
    pub extra: &'static [ExtraFile],
    /// Post-extraction build step, for sources that don't ship ready-made
    /// hunspell files.
    pub build: Option<Build>,
}

pub enum Fetch {
    /// Download one archive to `archive/<name>.<ext>` and extract it to
    /// `source/<name>/`.
    Archive {
        url: &'static str,
        kind: ArchiveKind,
    },
    /// No archive: download these files directly into `source/<name>/`.
    Files(&'static [ExtraFile]),
}

pub struct ExtraFile {
    pub url: &'static str,
    /// Destination path relative to `source/<name>/`.
    pub dest: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArchiveKind {
    /// Extracted as-is (no path components stripped). `.oxt`/`.xpi` files are
    /// also plain zips.
    Zip,
    /// Extracted with the first path component stripped
    /// (`tar --strip-components=1`).
    TarGz,
    /// Extracted with the first path component stripped.
    TarBz2,
}

pub enum Build {
    /// `make <targets...>` in `source/<name>/<subdir>`.
    Make {
        subdir: &'static str,
        targets: &'static [&'static str],
    },
    /// `./configure` (only when no `Makefile` exists yet) followed by
    /// `make <targets...>`, with `PERL5LIB` including `.` (hebrew).
    ConfigureMake {
        subdir: &'static str,
        targets: &'static [&'static str],
    },
    /// Extract further zip archives found inside the source directory:
    /// `(archive path, destination dir)`, both relative to `source/<name>/`
    /// (norwegian).
    NestedZips {
        zips: &'static [(&'static str, &'static str)],
    },
}

/// One generated dictionary under `dictionaries/<code>/`.
pub struct Dictionary {
    /// BCP-47 tag, also the output directory name, e.g. `"de-AT"`.
    pub code: &'static str,
    /// Human-readable language name, e.g. `"German (Austria)"`.
    pub name: &'static str,
    /// `Source.name` this dictionary is generated from.
    pub source: &'static str,
    /// The `.aff` file within `source/<source>/`.
    pub aff: SourceFile,
    /// The `.dic` file within `source/<source>/`.
    pub dic: SourceFile,
    /// License file within `source/<source>/`, when the source ships one.
    pub license: Option<SourceFile>,
    /// SPDX license expression.
    pub spdx: &'static str,
}

pub struct SourceFile {
    /// Path relative to `source/<source>/`.
    pub path: &'static str,
    pub encoding: Enc,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Enc {
    Utf8,
    /// True ISO 8859-1 (every byte maps to the same code point), unlike the
    /// WHATWG `iso-8859-1` label which aliases windows-1252.
    Latin1,
    Iso8859_2,
    Iso8859_15,
    Cp1251,
    Cp1252,
}

impl Source {
    pub fn by_name(name: &str) -> Option<&'static Source> {
        SOURCES.iter().find(|s| s.name == name)
    }
}

impl Dictionary {
    pub fn by_code(code: &str) -> Option<&'static Dictionary> {
        DICTIONARIES.iter().find(|d| d.code == code)
    }
}

pub static SOURCES: &[Source] = &[
    // armenian-eastern / armenian-western (-> frozen `hy` / `hyw`):
    // The Google Sites page now redirects to a Google sign-in wall (checked
    // 2026-06). Old URLs, in case it ever comes back:
    //   page: https://sites.google.com/site/araktransfer/home/spell-checkers
    //   eastern: .../hy_AM_e_1940_dict-1.1.oxt (zip)
    //   western: .../hy_AM_western-1.0.oxt (zip)
    // TODO: https://github.com/hyspell/HySpell_3.0.1/issues/1
    // western: hy-arevmda -> BCP now recommends hyw (always non-reformed)
    // eastern: hy-arevela -> BCP now recommends hy (always reformed, except in Iran)
    // See also: https://www.evnreport.com/raw-unfiltered/international-recognition-for-the-western-armenian-language
    // See: http://xuxen.eus/deskargatu, click “hunspell”, see the yellow thing.
    Source {
        name: "basque",
        page: "http://xuxen.eus/eu/home",
        fetch: Fetch::Archive {
            url: "http://xuxen.eus/static/hunspell/xuxen_5.1_hunspell.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing:
    Source {
        name: "breton",
        page: "https://github.com/Drouizig/hunspell-br",
        fetch: Fetch::Archive {
            url: "https://github.com/Drouizig/hunspell-br/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // See <http://bgoffice.sourceforge.net/ooo/index.html>, pick the *second* `тук` (which goes to “Bulgarian language support Files”)
    Source {
        name: "bulgarian",
        page: "http://bgoffice.sourceforge.net",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/bgoffice/OpenOffice.org%20Full%20Pack/4.3/OOo-full-pack-bg-4.3.zip?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // See <https://github.com/Softcatala/catalan-dict-tools/releases>, pick the latest.
    Source {
        name: "catalan",
        page: "https://github.com/Softcatala/catalan-dict-tools",
        fetch: Fetch::Archive {
            url: "https://github.com/Softcatala/catalan-dict-tools/releases/download/v3.0.8/ca-hunspell.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Same as `catalan`
    Source {
        name: "catalan-valencian",
        page: "https://github.com/Softcatala/catalan-dict-tools",
        fetch: Fetch::Archive {
            url: "https://github.com/Softcatala/catalan-dict-tools/releases/download/v3.0.8/ca-valencia-hunspell.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing:
    Source {
        name: "croatian",
        page: "https://github.com/krunose/hunspell-hr",
        fetch: Fetch::Archive {
            url: "https://github.com/krunose/hunspell-hr/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // See: <http://www.translatoblog.cz/hunspell/> (see `tady`). Or use a custom Google search on
    // that site to look for “hunspell” for recent entries?
    Source {
        name: "czech",
        page: "http://www.translatoblog.cz/hunspell/",
        fetch: Fetch::Archive {
            url: "http://www.translatoblog.cz/wp-content/uploads/2021/03/hunspell_cs.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // See <https://stavekontrolden.dk/?dictionaries=1>, get the OXT link.
    Source {
        name: "danish",
        page: "https://stavekontrolden.dk",
        fetch: Fetch::Archive {
            url: "https://stavekontrolden.dk/dictionaries/da_DK/da_DK-2.8.034.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing:
    Source {
        name: "dutch",
        page: "https://github.com/OpenTaal/opentaal-hunspell",
        fetch: Fetch::Archive {
            url: "https://github.com/OpenTaal/opentaal-hunspell/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to the link, get the latest.
    Source {
        name: "english",
        page: "https://extensions.openoffice.org/en/project/english-dictionaries-apache-openoffice",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/aoo-extensions/17102/96/dict-en-20231101_aoo.oxt?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to the link, go to “Additional Hunspell Dictionaries”, go to “Parent folder”.
    Source {
        name: "english-gb",
        page: "http://wordlist.aspell.net/dicts/",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/wordlist/speller/2020.12.07/hunspell-en_GB-ise-2020.12.07.zip?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Same as `english-gb`
    Source {
        name: "english-american",
        page: "http://wordlist.aspell.net/dicts/",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/wordlist/speller/2020.12.07/hunspell-en_US-2020.12.07.zip?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Same as `english-gb`
    Source {
        name: "english-canadian",
        page: "http://wordlist.aspell.net/dicts/",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/wordlist/speller/2020.12.07/hunspell-en_CA-2020.12.07.zip?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Same as `english-gb`
    Source {
        name: "english-australian",
        page: "http://wordlist.aspell.net/dicts/",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/wordlist/speller/2020.12.07/hunspell-en_AU-2020.12.07.zip?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Hasn’t updated in 20 years 🤷‍♂️
    Source {
        name: "esperanto",
        page: "http://www.esperantilo.org/index_en.html",
        fetch: Fetch::Archive {
            url: "http://www.esperantilo.org/evortaro.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // faroese (-> frozen `fo`):
    // https://stava.glasir.fo/download/hunspell.zip now returns the school's
    // homepage HTML for every path (checked 2026-06).
    // Go to <https://grammalecte.net/#download>, copy the url of “GRAMMALECTE pour LibreOffice 5.3+”.
    Source {
        name: "french",
        page: "https://grammalecte.net",
        fetch: Fetch::Archive {
            url: "https://grammalecte.net/oxt/Grammalecte-fr-v2.1.2.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing:
    Source {
        name: "frisian",
        page: "https://github.com/PanderMusubi/frisian",
        fetch: Fetch::Archive {
            url: "https://github.com/PanderMusubi/frisian/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // friulian (-> frozen `fur`):
    // http://digilander.libero.it/paganf/coretors/myspell-fur-12092005.zip
    // returns HTTP 410 Gone (checked 2026-06).
    // Fork addition (not in upstream wooorm/dictionaries): Ancient Greek,
    // based on the Perseus Project's Morpheus word list.
    Source {
        name: "ancient-greek",
        page: "https://github.com/mrakia/hunspell-ancient-greek",
        fetch: Fetch::Archive {
            url: "https://github.com/mrakia/hunspell-ancient-greek/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing:
    Source {
        name: "gaelic",
        page: "https://github.com/kscanne/hunspell-gd",
        fetch: Fetch::Archive {
            url: "https://github.com/kscanne/hunspell-gd/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: Some(Build::Make {
            subdir: "hunspell-gd-master",
            targets: &["gd_GB.dic", "gd_GB.aff"],
        }),
    },
    // Nothing:
    Source {
        name: "galician",
        page: "https://github.com/meixome/hunspell-gl",
        fetch: Fetch::Archive {
            url: "https://github.com/meixome/hunspell-gl/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[
            ExtraFile {
                url: "https://github.com/meixome/hunspell-gl/releases/download/18.07/gl_ES.aff",
                dest: "hunspell-gl-master/gl_ES.aff",
            },
            ExtraFile {
                url: "https://github.com/meixome/hunspell-gl/releases/download/18.07/gl_ES.dic",
                dest: "hunspell-gl-master/gl_ES.dic",
            },
        ],
        build: None,
    },
    // Nothing:
    Source {
        name: "georgian",
        page: "https://github.com/gamag/ka_GE.spell",
        fetch: Fetch::Archive {
            url: "https://github.com/gamag/ka_GE.spell/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Not updated in 6 years.
    // Go to <https://www.j3e.de/ispell/igerman98/dict/>, get the latest
    // `igerman98-20*` tarball.
    Source {
        name: "german",
        page: "https://www.j3e.de/ispell/igerman98/index_en.html",
        fetch: Fetch::Archive {
            url: "https://j3e.de/ispell/igerman98/dict/igerman98-20161207.tar.bz2",
            kind: ArchiveKind::TarBz2,
        },
        extra: &[],
        build: Some(Build::Make {
            subdir: "",
            targets: &["hunspell-all"],
        }),
    },
    // Nothing:
    Source {
        name: "greek",
        page: "https://github.com/stevestavropoulos/elspell",
        fetch: Fetch::Archive {
            url: "https://github.com/stevestavropoulos/elspell/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: Some(Build::Make {
            subdir: "elspell-master",
            targets: &[],
        }),
    },
    // Go to <https://thepolytonicproject.gr/spell/>, click the image/button:
    // “Ἐγκατάσταση σὲ OpenOffice / LibreOffice”, and get the latest from sourceforge.
    Source {
        name: "greek-polyton",
        page: "https://thepolytonicproject.gr/spell",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/greekpolytonicsp/greek_polytonic_2.0.7.oxt?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <http://hspell.ivrix.org.il/download.html>, copy/paste the URL of the
    // latest release
    Source {
        name: "hebrew",
        page: "http://hspell.ivrix.org.il",
        fetch: Fetch::Archive {
            url: "http://hspell.ivrix.org.il/hspell-1.4.tar.gz",
            kind: ArchiveKind::TarGz,
        },
        extra: &[],
        build: Some(Build::ConfigureMake {
            subdir: "",
            targets: &["hunspell"],
        }),
    },
    // TODO, this is impossible to build? <https://github.com/laszlonemeth/magyarispell/issues/9>
    // For now, go to: <https://github.com/crash5/mozilla-hungarian-spellchecker/releases>,
    // copy/paste the URL of the latest `.zip` file.
    Source {
        name: "hungarian",
        page: "https://github.com/laszlonemeth/magyarispell",
        fetch: Fetch::Archive {
            url: "https://github.com/crash5/mozilla-hungarian-spellchecker/releases/download/2024.03.28.01.14/MagyarIspell_3aa21cc.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Hasn’t updated in 8 years.
    // Go to <https://addons.thunderbird.net/en-US/thunderbird/addon/dict-ia/>,
    // copy/paste the URL for “Download Now” (but clean it).
    Source {
        name: "interlingua",
        page: "https://addons.thunderbird.net/en-US/thunderbird/addon/dict-ia/",
        fetch: Fetch::Archive {
            url: "https://addons.thunderbird.net/user-media/addons/_attachments/514646/interlingua_sownik_ortograficzny-2014.05.30-tb+fx.xpi",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing:
    Source {
        name: "interlingue",
        page: "https://github.com/Carmina16/hunspell-ie",
        fetch: Fetch::Archive {
            url: "https://github.com/Carmina16/hunspell-ie/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://github.com/kscanne/gaelspell/releases>, get the latest.
    // Note: hasn’t been a release in 4 years but 5.2 is planned apparently.
    Source {
        name: "irish",
        page: "https://github.com/kscanne/gaelspell",
        fetch: Fetch::Archive {
            url: "https://github.com/kscanne/gaelspell/releases/download/v5.1/hunspell-ga-5.1.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Down, unfortunately :'(
    // Source {
    //     name: "italian",
    //     page: "http://www.plio.it",
    //     fetch: Fetch::Archive {
    //         url: "https://master.dl.sourceforge.net/project/aoo-extensions/1204/14/dict-it.oxt?viasf=1",
    //         kind: ArchiveKind::Zip,
    //     },
    //     extra: &[],
    //     build: None,
    // },
    // Nothing:
    Source {
        name: "kinyarwanda",
        page: "https://github.com/kscanne/hunspell-rw",
        fetch: Fetch::Archive {
            url: "https://github.com/kscanne/hunspell-rw/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: Some(Build::Make {
            subdir: "hunspell-rw-master",
            targets: &[],
        }),
    },
    // Nothing:
    Source {
        name: "klingon",
        page: "https://github.com/PanderMusubi/klingon",
        fetch: Fetch::Archive {
            url: "https://github.com/PanderMusubi/klingon/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://github.com/spellcheck-ko/hunspell-dict-ko/releases>,
    // copy/paste the URL of the latest `.zip`.
    Source {
        name: "korean",
        page: "https://github.com/spellcheck-ko/hunspell-dict-ko",
        fetch: Fetch::Archive {
            url: "https://github.com/spellcheck-ko/hunspell-dict-ko/releases/download/0.7.94/ko-aff-dic-0.7.94.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <http://dict.dv.lv/download.php?prj=la>, copy the url of the latest `.oxt`.
    Source {
        name: "latgalian",
        page: "http://dict.dv.lv/home.php?prj=la",
        fetch: Fetch::Archive {
            url: "http://dict.dv.lv/download/ltg_LV-0.1.5.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <http://dict.dv.lv/download.php?prj=lv>, copy the url of the latest `.oxt`.
    Source {
        name: "latvian",
        page: "http://dict.dv.lv/home.php?prj=lv",
        fetch: Fetch::Archive {
            url: "http://dict.dv.lv/download/lv_LV-1.5.0.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Hasn’t been updates in 10 years.
    Source {
        name: "latin",
        page: "https://extensions.openoffice.org/project/dict-la",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/aoo-extensions/1141/3/dict-la_2013-03-31.oxt?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing.
    Source {
        name: "libreoffice",
        page: "https://github.com/LibreOffice/dictionaries",
        fetch: Fetch::Archive {
            url: "https://github.com/LibreOffice/dictionaries/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://github.com/ispell-lt/ispell-lt/releases/>,
    // get the latest myspell.
    Source {
        name: "lithuanian",
        page: "https://github.com/ispell-lt/ispell-lt",
        fetch: Fetch::Archive {
            url: "https://github.com/ispell-lt/ispell-lt/releases/download/rel-1.3.2/myspell-lt-1.3.2.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing.
    Source {
        name: "low-german",
        page: "https://github.com/tdf/dict_nds",
        fetch: Fetch::Archive {
            url: "https://github.com/tdf/dict_nds/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: Some(Build::Make {
            subdir: "dict_nds-master",
            targets: &["nds_de.aff", "nds_de.dic"],
        }),
    },
    // Nothing.
    Source {
        name: "luxembourgish",
        page: "https://github.com/spellchecker-lu/dictionary-lb-lu",
        fetch: Fetch::Archive {
            url: "https://github.com/spellchecker-lu/dictionary-lb-lu/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // It was taken offline unfortunately :'(
    // (Its build also ran `bash ./build_release.sh` in `hunspell-mk-master`.)
    // Source {
    //     name: "macedonian",
    //     page: "https://github.com/dimztimz/hunspell-mk",
    //     fetch: Fetch::Archive {
    //         url: "https://github.com/dimztimz/hunspell-mk/archive/master.zip",
    //         kind: ArchiveKind::Zip,
    //     },
    //     extra: &[],
    //     build: None,
    // },
    // Nothing:
    Source {
        name: "mongolian",
        page: "https://github.com/bataak/dict-mn",
        fetch: Fetch::Archive {
            url: "https://github.com/bataak/dict-mn/archive/main.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Hasn’t been updated in 10 years.
    Source {
        name: "nepali",
        page: "http://ltk.org.np",
        fetch: Fetch::Archive {
            url: "http://ltk.org.np/downloads/ne_NP_dict.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Website offline :'(
    Source {
        name: "norwegian",
        page: "http://no.speling.org",
        fetch: Fetch::Archive {
            url: "https://alioth-archive.debian.org/releases/spell-norwegian/spell-norwegian/spell-norwegian-latest.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: Some(Build::NestedZips {
            zips: &[
                ("no_NO-pack2-2.2.zip", "no"),
                ("no/nb_NO.zip", "nb"),
                ("no/nn_NO.zip", "nn"),
            ],
        }),
    },
    // Go to <https://gitlab.com/taissou/hunspell-files-for-occitan-lengadocian>,
    // click the `corrector_occitan_lengadocian_….oxt` (last?), copy/paste the “Download” URL
    Source {
        name: "occitan",
        page: "https://gitlab.com/taissou/hunspell-files-for-occitan-lengadocian",
        fetch: Fetch::Archive {
            url: "https://gitlab.com/taissou/hunspell-files-for-occitan-lengadocian/-/raw/master/corrector_occitan_lengadocian_1-2.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://github.com/b00f/lilak/releases>, get the latest `fa-IR.zip`.
    Source {
        name: "persian",
        page: "https://github.com/b00f/lilak",
        fetch: Fetch::Archive {
            url: "https://github.com/b00f/lilak/releases/download/v3.3/fa-IR.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Unknown, I contacted an old maintainer.
    Source {
        name: "polish",
        page: "http://extensions.openoffice.org/en/project/polish-dictionary-pack",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/aoo-extensions/806/4/pl-dict.oxt?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://natura.di.uminho.pt/download/sources/Dictionaries/hunspell/>,
    // get the last one **before** `hunspell-pt_PT-preao*`
    Source {
        name: "portuguese-pt",
        page: "https://natura.di.uminho.pt",
        fetch: Fetch::Archive {
            url: "https://natura.di.uminho.pt/download/sources/Dictionaries/hunspell/hunspell-pt_PT-20220621.tar.gz",
            kind: ArchiveKind::TarGz,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://rospell.wordpress.com/download/>,
    // get the hunspell one.
    Source {
        name: "romanian",
        page: "https://rospell.wordpress.com",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/rospell/Romanian%20dictionaries/dict-3.3.10/ro_RO.3.3.10.zip?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Not updated in 10 years.
    // Go to <https://bitbucket.org/Shaman_Alex/russian-dictionary-hunspell/downloads/>,
    // (or find the link to that on <https://code.google.com/archive/p/hunspell-ru/>).
    // Fork addition (not in upstream wooorm/dictionaries): Sanskrit, from the
    // same author's hindi-hunspell collection (also ships Hindi and Marathi).
    Source {
        name: "sanskrit",
        page: "https://github.com/Shreeshrii/hindi-hunspell",
        fetch: Fetch::Archive {
            url: "https://github.com/Shreeshrii/hindi-hunspell/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing.
    Source {
        name: "serbian",
        page: "https://github.com/grakic/hunspell-sr",
        fetch: Fetch::Archive {
            url: "https://github.com/grakic/hunspell-sr/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Hasn’t been updated in 10 years, but:
    // Go to <http://www.sk-spell.sk.cx/hunspell-sk>, get the most recent `hunspell-sk`.
    Source {
        name: "slovak",
        page: "http://www.sk-spell.sk.cx",
        fetch: Fetch::Archive {
            url: "http://www.sk-spell.sk.cx/file_download/92/hunspell-sk-20110228.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://extensions.libreoffice.org/extensions/slovenian-dictionary-pack/>,
    // get the top “download”.
    Source {
        name: "slovenian",
        page: "https://extensions.libreoffice.org/extensions/slovenian-dictionary-pack/",
        fetch: Fetch::Archive {
            url: "https://extensions.libreoffice.org/assets/downloads/752/1672786274/pack-sl.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://github.com/sbosio/rla-es/releases>,
    // hopefully only the version number changes 😅
    // Note: this source is not referenced by any dictionary (crawl.sh generates
    // `es` from `spanish-es` — spelled `spanish-ES` there, which only matched on
    // case-insensitive file systems).
    Source {
        name: "spanish",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-ar",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_AR.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-bo",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_BO.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-cl",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_CL.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-co",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_CO.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-cr",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_CR.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-cu",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_CU.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-do",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_DO.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-ec",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_EC.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-es",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_ES.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-gt",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_GT.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-hn",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_HN.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-mx",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_MX.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-ni",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_NI.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-pa",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_PA.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-pe",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_PE.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-ph",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_PH.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-pr",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_PR.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-py",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_PY.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-sv",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_SV.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-us",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_US.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-uy",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_UY.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    Source {
        name: "spanish-ve",
        page: "https://github.com/sbosio/rla-es",
        fetch: Fetch::Archive {
            url: "https://github.com/sbosio/rla-es/releases/download/v2.8/es_VE.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Hasn’t been updated in 4 years, but:
    // Go to <https://extensions.libreoffice.org/extensions/swedish-spelling-dictionary-den-stora-svenska-ordlistan>,
    // get the top “download”.
    Source {
        name: "swedish",
        page: "https://extensions.libreoffice.org/extensions/swedish-spelling-dictionary-den-stora-svenska-ordlistan",
        fetch: Fetch::Archive {
            url: "https://extensions.libreoffice.org/assets/downloads/z/ooo-swedish-dict-2-42.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Hasn’t been updated in 7 years, but:
    // Go to <http://extensions.openoffice.org/en/project/turkish-spellcheck-dictionary>,
    // get the top.
    Source {
        name: "turkish",
        page: "http://extensions.openoffice.org/en/project/turkish-spellcheck-dictionary",
        fetch: Fetch::Archive {
            url: "https://master.dl.sourceforge.net/project/aoo-extensions/18079/0/oo-turkish-dict-v1.3.oxt?viasf=1",
            kind: ArchiveKind::Zip,
        },
        extra: &[ExtraFile {
            url: "https://raw.githubusercontent.com/hrzafer/hunspell-tr/master/LICENSE",
            dest: "license",
        }],
        build: None,
    },
    // Nothing.
    Source {
        name: "turkmen",
        page: "https://github.com/nazartm/turkmen-spell-check-dictionary",
        fetch: Fetch::Archive {
            url: "https://github.com/nazartm/turkmen-spell-check-dictionary/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Go to <https://github.com/brown-uk/dict_uk/releases>,
    // get the top `hunspell-uk`.
    Source {
        name: "ukrainian",
        page: "https://github.com/brown-uk/dict_uk",
        fetch: Fetch::Archive {
            url: "https://github.com/brown-uk/dict_uk/releases/download/v6.4.4/hunspell-uk_UA_6.4.4.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[ExtraFile {
            url: "https://raw.githubusercontent.com/brown-uk/dict_uk/master/LICENSE",
            dest: "license",
        }],
        build: None,
    },
    // Hasn’t updated in 9 years, but:
    // Go to <https://github.com/1ec5/hunspell-vi/releases>.
    Source {
        name: "vietnamese",
        page: "https://github.com/1ec5/hunspell-vi",
        fetch: Fetch::Archive {
            url: "https://github.com/1ec5/hunspell-vi/releases/download/v2.2.0/vi_spellchecker_OOo3.oxt",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Nothing.
    Source {
        name: "welsh-gb",
        page: "https://github.com/techiaith/hunspell-cy",
        fetch: Fetch::Archive {
            url: "https://github.com/techiaith/hunspell-cy/archive/master.zip",
            kind: ArchiveKind::Zip,
        },
        extra: &[],
        build: None,
    },
    // Not an archive: two bare files (from the BUILD section of crawl.sh).
    Source {
        name: "estonian",
        page: "http://www.meso.ee/~jjpp/speller",
        fetch: Fetch::Files(&[
            ExtraFile {
                url: "http://www.meso.ee/~jjpp/speller/et_EE.aff",
                dest: "et.aff",
            },
            ExtraFile {
                url: "http://www.meso.ee/~jjpp/speller/et_EE.dic",
                dest: "et.dic",
            },
        ]),
        extra: &[],
        build: None,
    },
];

pub static DICTIONARIES: &[Dictionary] = &[
    Dictionary {
        code: "bg",
        name: "Bulgarian",
        source: "bulgarian",
        aff: SourceFile {
            path: "OOo-full-pack-bg-4.3/bg_BG.aff",
            encoding: Enc::Cp1251,
        },
        dic: SourceFile {
            path: "OOo-full-pack-bg-4.3/bg_BG.dic",
            encoding: Enc::Cp1251,
        },
        license: Some(SourceFile {
            path: "OOo-full-pack-bg-4.3/README_spell.bulgarian",
            encoding: Enc::Cp1251,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "br",
        name: "Breton",
        source: "breton",
        aff: SourceFile {
            path: "hunspell-br-master/br_FR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-br-master/br_FR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-br-master/README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "ca",
        name: "Catalan",
        source: "catalan",
        aff: SourceFile {
            path: "catalan.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "catalan.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1)",
    },
    Dictionary {
        code: "ca-valencia",
        name: "Catalan (Valencia)",
        source: "catalan-valencian",
        aff: SourceFile {
            path: "catalan-valencia.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "catalan-valencia.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1)",
    },
    Dictionary {
        code: "cs",
        name: "Czech",
        source: "czech",
        aff: SourceFile {
            path: "cs_CZ.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "cs_CZ.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "readme.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "da",
        name: "Danish",
        source: "danish",
        aff: SourceFile {
            path: "da_DK.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "da_DK.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_da_DK.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "de",
        name: "German",
        source: "german",
        aff: SourceFile {
            path: "hunspell/de_DE.aff",
            encoding: Enc::Latin1,
        },
        dic: SourceFile {
            path: "hunspell/de_DE.dic",
            encoding: Enc::Latin1,
        },
        license: Some(SourceFile {
            path: "hunspell/Copyright",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR GPL-3.0)",
    },
    Dictionary {
        code: "de-AT",
        name: "German (Austria)",
        source: "german",
        aff: SourceFile {
            path: "hunspell/de_AT.aff",
            encoding: Enc::Latin1,
        },
        dic: SourceFile {
            path: "hunspell/de_AT.dic",
            encoding: Enc::Latin1,
        },
        license: Some(SourceFile {
            path: "hunspell/Copyright",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR GPL-3.0)",
    },
    Dictionary {
        code: "de-CH",
        name: "German (Switzerland)",
        source: "german",
        aff: SourceFile {
            path: "hunspell/de_CH.aff",
            encoding: Enc::Latin1,
        },
        dic: SourceFile {
            path: "hunspell/de_CH.dic",
            encoding: Enc::Latin1,
        },
        license: Some(SourceFile {
            path: "hunspell/Copyright",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR GPL-3.0)",
    },
    Dictionary {
        code: "el",
        name: "Greek",
        source: "greek",
        aff: SourceFile {
            path: "elspell-master/myspell/el_GR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "elspell-master/myspell/el_GR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "elspell-master/myspell/README_el_GR.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "el-polyton",
        name: "Greek (Polyton)",
        source: "greek-polyton",
        aff: SourceFile {
            path: "el_GR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "el_GR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_el_GR.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    // Note that “the Hunspell English Dictionaries” are very vaguely licensed.
    // Read more in the license file. Note that the SPDX “(MIT AND BSD)”
    // comes from aspell’s description as “BSD/MIT-like”.
    // See: http://wordlist.aspell.net/other-dicts/#official
    Dictionary {
        code: "en-AU",
        name: "English (Australia)",
        source: "english-australian",
        aff: SourceFile {
            path: "en_AU.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "en_AU.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_en_AU.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(MIT AND BSD)",
    },
    Dictionary {
        code: "en-CA",
        name: "English (Canada)",
        source: "english-canadian",
        aff: SourceFile {
            path: "en_CA.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "en_CA.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_en_CA.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(MIT AND BSD)",
    },
    Dictionary {
        code: "en-GB",
        name: "English (United Kingdom)",
        source: "english-gb",
        aff: SourceFile {
            path: "en_GB-ise.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "en_GB-ise.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_en_GB-ise.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(MIT AND BSD)",
    },
    Dictionary {
        code: "en",
        name: "English",
        source: "english-american",
        aff: SourceFile {
            path: "en_US.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "en_US.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_en_US.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(MIT AND BSD)",
    },
    Dictionary {
        code: "en-ZA",
        name: "English (South Africa)",
        source: "english",
        aff: SourceFile {
            path: "en_ZA.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "en_ZA.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_en_ZA.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "LGPL-2.1",
    },
    Dictionary {
        code: "eo",
        name: "Esperanto",
        source: "esperanto",
        aff: SourceFile {
            path: "eo_ilo.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "eo_ilo.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSE.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-2.0",
    },
    // crawl.sh spells this source `spanish-ES`; it only resolved because the
    // file system is case-insensitive. The crawled source is `spanish-es`.
    Dictionary {
        code: "es",
        name: "Spanish",
        source: "spanish-es",
        aff: SourceFile {
            path: "es_ES.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_ES.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-AR",
        name: "Spanish (Argentina)",
        source: "spanish-ar",
        aff: SourceFile {
            path: "es_AR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_AR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-BO",
        name: "Spanish (Bolivia)",
        source: "spanish-bo",
        aff: SourceFile {
            path: "es_BO.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_BO.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-CL",
        name: "Spanish (Chile)",
        source: "spanish-cl",
        aff: SourceFile {
            path: "es_CL.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_CL.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-CO",
        name: "Spanish (Colombia)",
        source: "spanish-co",
        aff: SourceFile {
            path: "es_CO.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_CO.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-CR",
        name: "Spanish (Costa Rica)",
        source: "spanish-cr",
        aff: SourceFile {
            path: "es_CR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_CR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-CU",
        name: "Spanish (Cuba)",
        source: "spanish-cu",
        aff: SourceFile {
            path: "es_CU.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_CU.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-DO",
        name: "Spanish (Dominican Republic)",
        source: "spanish-do",
        aff: SourceFile {
            path: "es_DO.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_DO.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-EC",
        name: "Spanish (Ecuador)",
        source: "spanish-ec",
        aff: SourceFile {
            path: "es_EC.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_EC.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-GT",
        name: "Spanish (Guatemala)",
        source: "spanish-gt",
        aff: SourceFile {
            path: "es_GT.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_GT.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-HN",
        name: "Spanish (Honduras)",
        source: "spanish-hn",
        aff: SourceFile {
            path: "es_HN.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_HN.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-MX",
        name: "Spanish (Mexico)",
        source: "spanish-mx",
        aff: SourceFile {
            path: "es_MX.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_MX.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-NI",
        name: "Spanish (Nicaragua)",
        source: "spanish-ni",
        aff: SourceFile {
            path: "es_NI.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_NI.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-PA",
        name: "Spanish (Panama)",
        source: "spanish-pa",
        aff: SourceFile {
            path: "es_PA.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_PA.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-PE",
        name: "Spanish (Peru)",
        source: "spanish-pe",
        aff: SourceFile {
            path: "es_PE.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_PE.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-PH",
        name: "Spanish (Philippines)",
        source: "spanish-ph",
        aff: SourceFile {
            path: "es_PH.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_PH.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-PR",
        name: "Spanish (Puerto Rico)",
        source: "spanish-pr",
        aff: SourceFile {
            path: "es_PR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_PR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-PY",
        name: "Spanish (Paraguay)",
        source: "spanish-py",
        aff: SourceFile {
            path: "es_PY.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_PY.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-SV",
        name: "Spanish (El Salvador)",
        source: "spanish-sv",
        aff: SourceFile {
            path: "es_SV.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_SV.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-US",
        name: "Spanish (United States of America)",
        source: "spanish-us",
        aff: SourceFile {
            path: "es_US.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_US.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-UY",
        name: "Spanish (Uruguay)",
        source: "spanish-uy",
        aff: SourceFile {
            path: "es_UY.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_UY.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "es-VE",
        name: "Spanish (Venezuela)",
        source: "spanish-ve",
        aff: SourceFile {
            path: "es_VE.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "es_VE.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-1.1)",
    },
    Dictionary {
        code: "et",
        name: "Estonian",
        source: "estonian",
        aff: SourceFile {
            path: "et.aff",
            encoding: Enc::Iso8859_15,
        },
        dic: SourceFile {
            path: "et.dic",
            encoding: Enc::Iso8859_15,
        },
        license: None,
        spdx: "LGPL-2.1",
    },
    Dictionary {
        code: "eu",
        name: "Basque",
        source: "basque",
        aff: SourceFile {
            path: "eu_ES.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "eu_ES.dic",
            encoding: Enc::Utf8,
        },
        license: None,
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "fa",
        name: "Persian",
        source: "persian",
        aff: SourceFile {
            path: "fa-IR/fa-IR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "fa-IR/fa-IR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "fa-IR/license",
            encoding: Enc::Utf8,
        }),
        spdx: "Apache-2.0",
    },
    // `fo` is frozen — see FROZEN below.
    // French: use classic (“classique”) because the readme suggests so.
    Dictionary {
        code: "fr",
        name: "French",
        source: "french",
        aff: SourceFile {
            path: "dictionaries/fr-classique.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries/fr-classique.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dictionaries/README_dict_fr.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "MPL-2.0",
    },
    // `fur` is frozen — see FROZEN below.
    Dictionary {
        code: "fy",
        name: "Western Frisian",
        source: "frisian",
        aff: SourceFile {
            path: "frisian-master/generated/fy_NL.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "frisian-master/generated/fy_NL.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "frisian-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    Dictionary {
        code: "ga",
        name: "Irish",
        source: "irish",
        aff: SourceFile {
            path: "ga_IE.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "ga_IE.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_ga_IE.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "gd",
        name: "Scottish Gaelic",
        source: "gaelic",
        aff: SourceFile {
            path: "hunspell-gd-master/gd_GB.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-gd-master/gd_GB.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-gd-master/README_gd_GB.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    Dictionary {
        code: "gl",
        name: "Galician",
        source: "galician",
        aff: SourceFile {
            path: "hunspell-gl-master/gl_ES.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-gl-master/gl_ES.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-gl-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    // Fork addition — see the `ancient-greek` source.
    Dictionary {
        code: "grc",
        name: "Ancient Greek",
        source: "ancient-greek",
        aff: SourceFile {
            path: "hunspell-ancient-greek-master/grc_GR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-ancient-greek-master/grc_GR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-ancient-greek-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    Dictionary {
        code: "he",
        name: "Hebrew",
        source: "hebrew",
        aff: SourceFile {
            path: "he.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "he.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "AGPL-3.0",
    },
    Dictionary {
        code: "hr",
        name: "Croatian",
        source: "croatian",
        aff: SourceFile {
            path: "hunspell-hr-master/hr_HR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-hr-master/hr_HR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-hr-master/README_hr_HR.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(LGPL-2.1 OR SISSL)",
    },
    // TODO: laszlonemeth/magyarispell#9
    Dictionary {
        code: "hu",
        name: "Hungarian",
        source: "hungarian",
        aff: SourceFile {
            path: "hu_HU.aff",
            encoding: Enc::Iso8859_2,
        },
        dic: SourceFile {
            path: "hu_HU.dic",
            encoding: Enc::Iso8859_2,
        },
        license: Some(SourceFile {
            path: "README.en",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    // `hy` and `hyw` are frozen — see FROZEN below.
    Dictionary {
        code: "ia",
        name: "Interlingua",
        source: "interlingua",
        aff: SourceFile {
            path: "dictionaries/ia.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries/ia.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dictionaries/README_dict-ia.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    Dictionary {
        code: "ie",
        name: "Interlingue",
        source: "interlingue",
        aff: SourceFile {
            path: "hunspell-ie-master/ie.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-ie-master/ie.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-ie-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "Apache-2.0",
    },
    Dictionary {
        code: "is",
        name: "Icelandic",
        source: "libreoffice",
        aff: SourceFile {
            path: "dictionaries-master/is/is.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries-master/is/is.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dictionaries-master/is/license.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "CC-BY-SA-3.0",
    },
    // Offline...
    // Dictionary {
    //     code: "it",
    //     name: "Italian",
    //     source: "italian",
    //     aff: SourceFile {
    //         path: "dictionaries/it_IT.aff",
    //         encoding: Enc::Utf8,
    //     },
    //     dic: SourceFile {
    //         path: "dictionaries/it_IT.dic",
    //         encoding: Enc::Utf8,
    //     },
    //     license: Some(SourceFile {
    //         path: "dictionaries/README.txt",
    //         encoding: Enc::Utf8,
    //     }),
    //     spdx: "GPL-3.0",
    // },
    Dictionary {
        code: "ka",
        name: "Georgian",
        source: "georgian",
        aff: SourceFile {
            path: "ka_GE.spell-master/dictionaries/ka_GE.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "ka_GE.spell-master/dictionaries/ka_GE.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "ka_GE.spell-master/LICENSE.mit",
            encoding: Enc::Utf8,
        }),
        spdx: "MIT",
    },
    // crawl.sh omits the license encoding argument here (a bug); the file is
    // UTF-8.
    Dictionary {
        code: "ko",
        name: "Korean",
        source: "korean",
        aff: SourceFile {
            path: "ko-aff-dic-0.7.94/ko.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "ko-aff-dic-0.7.94/ko.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "ko-aff-dic-0.7.94/LICENSE.md",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "la",
        name: "Latin",
        source: "latin",
        aff: SourceFile {
            path: "la/universal/la.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "la/universal/la.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "la/README_la.txt",
            encoding: Enc::Cp1252,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "lb",
        name: "Luxembourgish",
        source: "luxembourgish",
        aff: SourceFile {
            path: "dictionary-lb-lu-master/lb_LU.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionary-lb-lu-master/lb_LU.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dictionary-lb-lu-master/LICENSE.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "EUPL-1.1",
    },
    Dictionary {
        code: "lt",
        name: "Lithuanian",
        source: "lithuanian",
        aff: SourceFile {
            path: "myspell-lt-1.3.2/lt_LT.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "myspell-lt-1.3.2/lt_LT.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "myspell-lt-1.3.2/COPYING",
            encoding: Enc::Utf8,
        }),
        spdx: "BSD-3-Clause",
    },
    Dictionary {
        code: "ltg",
        name: "Latgalian",
        source: "latgalian",
        aff: SourceFile {
            path: "ltg_LV.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "ltg_LV.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_ltg_LV.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "LGPL-2.1",
    },
    Dictionary {
        code: "lv",
        name: "Latvian",
        source: "latvian",
        aff: SourceFile {
            path: "lv_LV.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "lv_LV.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_lv_LV.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "LGPL-2.1",
    },
    // Taken offline, like the `macedonian` source.
    // Dictionary {
    //     code: "mk",
    //     name: "Macedonian",
    //     source: "macedonian",
    //     aff: SourceFile {
    //         path: "hunspell-mk-master/release/mk.aff",
    //         encoding: Enc::Utf8,
    //     },
    //     dic: SourceFile {
    //         path: "hunspell-mk-master/release/mk.dic",
    //         encoding: Enc::Utf8,
    //     },
    //     license: Some(SourceFile {
    //         path: "hunspell-mk-master/release/LICENCE.txt",
    //         encoding: Enc::Utf8,
    //     }),
    //     spdx: "GPL-3.0",
    // },
    Dictionary {
        code: "mn",
        name: "Mongolian",
        source: "mongolian",
        aff: SourceFile {
            path: "dict-mn-main/mn_MN/mn_MN.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dict-mn-main/mn_MN/mn_MN.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dict-mn-main/mn_MN/README_mn_MN.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "LPPL-1.3c",
    },
    Dictionary {
        code: "ne",
        name: "Nepali",
        source: "nepali",
        aff: SourceFile {
            path: "ne_NP.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "ne_NP.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_ne_NP.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "LGPL-2.1",
    },
    Dictionary {
        code: "nb",
        name: "Norwegian Bokmål",
        source: "norwegian",
        aff: SourceFile {
            path: "nb/nb_NO.aff",
            encoding: Enc::Latin1,
        },
        dic: SourceFile {
            path: "nb/nb_NO.dic",
            encoding: Enc::Latin1,
        },
        license: Some(SourceFile {
            path: "nb/README_nb_NO.txt",
            encoding: Enc::Latin1,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "nds",
        name: "Low German",
        source: "low-german",
        aff: SourceFile {
            path: "dict_nds-master/nds_de.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dict_nds-master/nds_de.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dict_nds-master/README",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    // Dutch is down. They seem to be working on a new version.
    Dictionary {
        code: "nl",
        name: "Dutch",
        source: "dutch",
        aff: SourceFile {
            path: "opentaal-hunspell-master/nl.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "opentaal-hunspell-master/nl.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "opentaal-hunspell-master/LICENSE.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(BSD-3-Clause OR CC-BY-3.0)",
    },
    Dictionary {
        code: "nn",
        name: "Norwegian Nynorsk",
        source: "norwegian",
        aff: SourceFile {
            path: "nn/nn_NO.aff",
            encoding: Enc::Latin1,
        },
        dic: SourceFile {
            path: "nn/nn_NO.dic",
            encoding: Enc::Latin1,
        },
        license: Some(SourceFile {
            path: "nn/README_nn_NO.txt",
            encoding: Enc::Latin1,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "oc",
        name: "Occitan",
        source: "occitan",
        aff: SourceFile {
            path: "oc_FR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "oc_FR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSES-en.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "pl",
        name: "Polish",
        source: "polish",
        aff: SourceFile {
            path: "pl_PL.aff",
            encoding: Enc::Iso8859_2,
        },
        dic: SourceFile {
            path: "pl_PL.dic",
            encoding: Enc::Iso8859_2,
        },
        license: Some(SourceFile {
            path: "README_en.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-3.0 OR MPL-2.0)",
    },
    Dictionary {
        code: "pt-PT",
        name: "Portuguese (Portugal)",
        source: "portuguese-pt",
        aff: SourceFile {
            path: "pt_PT.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "pt_PT.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README_pt_PT.txt",
            encoding: Enc::Cp1252,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "pt",
        name: "Portuguese",
        source: "libreoffice",
        aff: SourceFile {
            path: "dictionaries-master/pt_BR/pt_BR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries-master/pt_BR/pt_BR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dictionaries-master/pt_BR/README_en.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(LGPL-3.0 OR MPL-2.0)",
    },
    Dictionary {
        code: "ro",
        name: "Romanian",
        source: "romanian",
        aff: SourceFile {
            path: "ro_RO.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "ro_RO.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "README",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "ru",
        name: "Russian",
        source: "libreoffice",
        aff: SourceFile {
            path: "dictionaries-master/ru_RU/ru_RU.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries-master/ru_RU/ru_RU.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "dictionaries-master/ru_RU/README_ru_RU.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "BSD-3-Clause",
    },
    Dictionary {
        code: "rw",
        name: "Kinyarwanda",
        source: "kinyarwanda",
        aff: SourceFile {
            path: "hunspell-rw-master/rw_RW.aff",
            encoding: Enc::Latin1,
        },
        dic: SourceFile {
            path: "hunspell-rw-master/rw_RW.dic",
            encoding: Enc::Latin1,
        },
        license: Some(SourceFile {
            path: "hunspell-rw-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    // Fork addition — see the `sanskrit` source.
    Dictionary {
        code: "sa",
        name: "Sanskrit",
        source: "sanskrit",
        aff: SourceFile {
            path: "hindi-hunspell-master/Sanskrit/sa_IN.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hindi-hunspell-master/Sanskrit/sa_IN.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hindi-hunspell-master/Sanskrit/COPYING",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "sk",
        name: "Slovak",
        source: "slovak",
        aff: SourceFile {
            path: "hunspell-sk-20110228/sk_SK.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-sk-20110228/sk_SK.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-sk-20110228/doc/Copyright",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
    },
    Dictionary {
        code: "sl",
        name: "Slovenian",
        source: "slovenian",
        aff: SourceFile {
            path: "sl_SI.aff",
            encoding: Enc::Iso8859_2,
        },
        dic: SourceFile {
            path: "sl_SI.dic",
            encoding: Enc::Iso8859_2,
        },
        license: Some(SourceFile {
            path: "README_sl_SI.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-3.0 OR LGPL-2.1)",
    },
    Dictionary {
        code: "sr",
        name: "Serbian",
        source: "serbian",
        aff: SourceFile {
            path: "hunspell-sr-master/sr.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-sr-master/sr.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-sr-master/README_sr.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1 OR CC-BY-SA-3.0)",
    },
    Dictionary {
        code: "sr-Latn",
        name: "Serbian (Latin script)",
        source: "serbian",
        aff: SourceFile {
            path: "hunspell-sr-master/sr-Latn.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-sr-master/sr-Latn.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-sr-master/README-sr-Latn.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1 OR CC-BY-SA-3.0)",
    },
    Dictionary {
        code: "sv",
        name: "Swedish",
        source: "swedish",
        aff: SourceFile {
            path: "dictionaries/sv_SE.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries/sv_SE.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSE_en_US.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "LGPL-3.0",
    },
    Dictionary {
        code: "sv-FI",
        name: "Swedish (Finland)",
        source: "swedish",
        aff: SourceFile {
            path: "dictionaries/sv_FI.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries/sv_FI.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSE_en_US.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "LGPL-3.0",
    },
    Dictionary {
        code: "tk",
        name: "Turkmen",
        source: "turkmen",
        aff: SourceFile {
            path: "turkmen-spell-check-dictionary-master/tk_TM.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "turkmen-spell-check-dictionary-master/tk_TM.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "turkmen-spell-check-dictionary-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "Apache-2.0",
    },
    Dictionary {
        code: "tlh",
        name: "Klingon",
        source: "klingon",
        aff: SourceFile {
            path: "klingon-master/generated/tlh.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "klingon-master/generated/tlh.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "klingon-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "Apache-2.0",
    },
    Dictionary {
        code: "tlh-Latn",
        name: "Klingon (Latin script)",
        source: "klingon",
        aff: SourceFile {
            path: "klingon-master/generated/tlh_Latn.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "klingon-master/generated/tlh_Latn.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "klingon-master/LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "Apache-2.0",
    },
    Dictionary {
        code: "tr",
        name: "Turkish",
        source: "turkish",
        aff: SourceFile {
            path: "dictionaries/tr-TR.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries/tr-TR.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "license",
            encoding: Enc::Utf8,
        }),
        spdx: "MIT",
    },
    Dictionary {
        code: "uk",
        name: "Ukrainian",
        source: "ukrainian",
        aff: SourceFile {
            path: "uk_UA.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "uk_UA.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSE",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-3.0",
    },
    Dictionary {
        code: "vi",
        name: "Vietnamese",
        source: "vietnamese",
        aff: SourceFile {
            path: "dictionaries/vi_VN.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "dictionaries/vi_VN.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "LICENSES-en.txt",
            encoding: Enc::Utf8,
        }),
        spdx: "GPL-2.0",
    },
    Dictionary {
        code: "cy",
        name: "Welsh",
        source: "welsh-gb",
        aff: SourceFile {
            path: "hunspell-cy-master/cy_GB.aff",
            encoding: Enc::Utf8,
        },
        dic: SourceFile {
            path: "hunspell-cy-master/cy_GB.dic",
            encoding: Enc::Utf8,
        },
        license: Some(SourceFile {
            path: "hunspell-cy-master/LICENCE",
            encoding: Enc::Utf8,
        }),
        spdx: "LGPL-3.0",
    },
];

/// A dictionary whose upstream source has disappeared: the checked-in output
/// is kept (and validated) as-is, but cannot be regenerated.
pub struct Frozen {
    pub code: &'static str,
    pub name: &'static str,
    pub spdx: &'static str,
    pub has_license: bool,
}

pub static FROZEN: &[Frozen] = &[
    // The faroese source <https://stava.glasir.fo/download/hunspell.zip> now
    // returns the school's homepage for every path.
    Frozen {
        code: "fo",
        name: "Faroese",
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
        has_license: true,
    },
    // The friulian source (digilander.libero.it) returns HTTP 410 Gone.
    Frozen {
        code: "fur",
        name: "Friulian",
        spdx: "GPL-2.0",
        has_license: true,
    },
    // The Armenian Google Sites source is behind a Google sign-in wall now.
    // Note: the checked-in `hy` includes the `hy` patch from src/patches.rs
    // (SFX VD rule count), applied in place.
    Frozen {
        code: "hy",
        name: "Armenian",
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
        has_license: true,
    },
    Frozen {
        code: "hyw",
        name: "Western Armenian",
        spdx: "(GPL-2.0 OR LGPL-2.1 OR MPL-1.1)",
        has_license: true,
    },
    // Upstream <http://linguistico.sourceforge.net/pages/dizionario_italiano.html>
    // went offline; last generated from its 2008 zip.
    Frozen {
        code: "it",
        name: "Italian",
        spdx: "GPL-3.0",
        has_license: true,
    },
    // Upstream <https://github.com/dimztimz/hunspell-mk> requires a build
    // step (`build_release.sh`) that upstream wooorm/dictionaries disabled;
    // kept from the last successful generation.
    Frozen {
        code: "mk",
        name: "Macedonian",
        spdx: "GPL-3.0",
        has_license: true,
    },
];
