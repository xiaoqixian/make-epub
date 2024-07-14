// Date:   Wed Jul 03 12:33:51 2024
// Mail:   lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

pub const ZH_NUMS: &[char] =
    &['零', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十'];
pub const ZH_UNITS: &[char] = &['零', '十', '百', '千', '万'];

pub const SUBDIRS: &[&'static str] = &["META-INF", "OPS/html", "OPS/css", "OPS/images"];

pub const MIMETYPE: &'static str = r"application/epub+zip";
pub const CONTAINER: &'static str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
   <rootfiles>
      <rootfile full-path="OPS/content.opf" media-type="application/oebps-package+xml"/>
   </rootfiles>
</container>"#;
pub const MAIN_CSS: &'static str = r"body {
    padding: 0%;
    margin-top: 0%;
    margin-bottom: 0%;
    margin-left: 1%;
    margin-right: 1%;
    line-height:130%;
    text-align: justify;
}
div {
    margin:0px;
    padding:0px;
    line-height:130%;
    text-align: justify;
}
p {
    text-align: justify;
    text-indent: 2em;
    line-height:130%;
}
.cover {
    width:100%;
    padding:0px;
}
.center {
    text-align: center;
    margin-left: 0%;
    margin-right: 0%;
}
.left {
    text-align: center;
    margin-left: 0%;
    margin-right: 0%;
}
.right {
    text-align: right;
    margin-left: 0%;
    margin-right: 0%;
}
.quote {
    margin-top: 0%;
    margin-bottom: 0%;
    margin-left: 1em;
    margin-right: 1em;
    text-align: justify;
}
h1 {
    line-height:130%;
    text-align: center;
    font-weight:bold;
    font-size:xx-large;
}
h2 {
    line-height:130%;
    text-align: center;
    font-weight:bold;
    font-size:x-large;
}
h3 {
    line-height:130%;
    text-align: center;
    font-weight:bold;
    font-size:large;
}
h4 {
    line-height:130%;
    text-align: center;
    font-weight:bold;
    font-size:medium;
}
h5 {
    line-height:130%;
    text-align: center;
    font-weight:bold;
    font-size:small;
}
h6 {
    line-height:130%;
    text-align: center;
    font-weight:bold;
    font-size:x-small;
}
h1.vol {
    font-size: 1.1em;
    width: 1em;
    margin: 30% auto 1em auto;
    text-align: center;
}
p.vol {
    font-size: 1.4em;
    margin: 0 auto;
    text-align: center;
    text-indent: 0em;
    duokan-text-indent: 0em;
    border-left: 1px solid #fff;
    border-right: 1px solid #fff;
    width: 1em;
    padding: 3px 4px;
    font-weight:bold;
}";

pub const COVER_CSS: &'static str = r"body {
  background-color: #FFFFFF;
  margin-bottom: 0px;
  margin-left: 0px;
  margin-right: 0px;
  margin-top: 0px;
  text-align: center;
}
img {
  max-height: 100%;
  max-width: 100%;
}";

#[macro_export]
macro_rules! NCX_header {
    ($title: expr) => {
        format_args!(r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE ncx PUBLIC "-//NISO//DTD ncx 2005-1//EN"
 "http://www.daisy.org/z3986/2005/ncx-2005-1.dtd">
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1">
  <head>
    <meta name="dtb:uid" content="urn:uuid:d0cca237-33a9-409b-acd7-4a5586862c1d" />
    <meta name="dtb:depth" content="2" />
    <meta name="dtb:totalPageCount" content="0" />
    <meta name="dtb:maxPageNumber" content="0" />
  </head>
  <docTitle>
    <text>{}</text>
  </docTitle>
  <navMap>
    <navPoint id="coverpage" playOrder="0">
      <navLabel><text>封面</text></navLabel>
      <content src="html/coverpage.html" />
    </navPoint>
"#, $title)
    }
}
#[macro_export]
macro_rules! NCX_volume_entry {
    ($title: expr, $order: expr, 1) => {
        format_args!(r#"
    <navPoint id="volume{}" playOrder="{}"}>
      <navLabel><text>{}</text></navLabel>
      <content src="html/volume{}.html" />"#,
            1,
            $order,
            $title,
            1
        )
    };
    ($title: expr, $order: expr, $vol_seq: expr) => {
        format_args!(r#"
    </navPoint>
    <navPoint id="volume{}" playOrder="{}">
      <navLabel><text>{}</text></navLabel>
      <content src="html/volume{}.html" />"#,
            $vol_seq,
            $order,
            $title,
            $vol_seq
        )
    }
}
#[macro_export]
macro_rules! NCX_chapter_entry {
    ($title: expr, $order: expr, $chap_seq: expr, $zh_seq: expr) => {
        format_args!(r#"
      <navPoint id="chapter{}" playOrder="{}">
        <navLabel><text>第{}章 {}</text></navLabel>
        <content src="html/chapter{}.html"/>
      </navPoint>"#,
            $chap_seq,
            $order,
            $zh_seq,
            $title,
            $chap_seq
        )
    }
}
pub const NCX_END: &'static str = r#"
    </navPoint>
  </navMap>
</ncx>"#;
pub const NCX_END_NO_VOLUME: &'static str = r#"
  </navMap>
</ncx>"#;
pub fn ncx_end(has_volume: bool) -> &'static str {
    if has_volume {
        NCX_END
    } else {
        NCX_END_NO_VOLUME
    }
}

#[macro_export]
macro_rules! OPF_header {
    ($name: expr, $author: expr, $cover_ext: expr) => {
        format_args!(r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<package xmlns="http://www.idpf.org/2007/opf" unique-identifier="duokan-book-id" version="2.0">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:opf="http://www.idpf.org/2007/opf">
    <dc:identifier id="duokan-book-id" opf:scheme="UUID">urn:uuid:d0cca237-33a9-409b-acd7-4a5586862c1d</dc:identifier>
    <dc:title>{}</dc:title>
    <dc:creator opf:role="aut">{}</dc:creator>
    <dc:language>zh-CN</dc:language>
    <dc:contributor opf:role="cre"></dc:contributor>
    <dc:publisher></dc:publisher>
    <dc:date opf:event="modification">2024-07-04</dc:date>
    <meta content="0.8.7" name="Sigil version" />
    <meta name="cover" content="cover.{}" />
  </metadata>
  <manifest>
    <item id="ncx" href="toc.ncx" media-type="application/x-dtbncx+xml" />
    <item id="coverpage" href="html/coverpage.html" media-type="application/xhtml+xml" />
    <item id="main-css" href="css/main.css" media-type="text/css" />
    <item id="css" href="css/main.css" media-type="text/css" />
    <item id="cover.{}" href="images/cover.{}" media-type="image/{}" />"#,
            $name,
            $author,
            $cover_ext,
            $cover_ext,
            $cover_ext,
            $cover_ext
        )
    }
}
#[macro_export]
macro_rules! OPF_item {
    (Volume, $seq: expr) => {
        format_args!(r#"
    <item id="volume{}" href="html/volume{}.html" media-type="application/xhtml+xml"/>"#,
            $seq,
            $seq
        )
    };
    (Chapter, $seq: expr) => {
        format_args!(r#"
    <item id="chapter{}" href="html/chapter{}.html" media-type="application/xhtml+xml"/>"#,
            $seq,
            $seq
        )
    }
}
pub const OPF_MID: &'static str = r#"
  </manifest>
  <spine toc="ncx">"#;

#[macro_export]
macro_rules! OPF_itemref {
    (Volume, $seq: expr) => {
        format_args!(r#"
    <itemref idref="volume{}" linear="yes"/>"#,
            $seq
        )
    };

    (Chapter, $seq: expr) => {
        format_args!(r#"
    <itemref idref="chapter{}" linear="yes"/>"#,
            $seq
        )
    }
}

pub const OPF_END: &'static str = r#"
  </spine>
  <guide>
    <reference href="html/coverpage.html" title="封面" type="cover" xmlns="http://www.idpf.org/2007/opf" />
  </guide>
</package>"#;

pub fn coverpage(cover_ext: &str) -> String {
    format!(r#"
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="zh-CN">
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
<title>bookcover</title>
</head>
<body>
<img class="cover" src="../images/cover.{}" alt="Cover Image Not Found"/>
</body>
</html>"#, cover_ext)
}

#[macro_export]
macro_rules! volume {
    ($seq: expr, $title: expr) => {
        format_args!(r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="zh-CN">
<head>
  <title></title>
  <link href="../css/main.css" rel="stylesheet" type="text/css" />
</head>
<body class="vol">
  <h1 class="vol" title="第{}卷 {}">第{}卷</h1>
  <p class="vol">{}</p>
</body>
</html>"#,
            $seq, $title, $seq, $title
        )
    }
}

#[macro_export]
macro_rules! chapter_header {
    ($title: expr) => {
        format_args!(r#"<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="zh-CN">
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
<link rel="stylesheet" type="text/css" href="../css/main.css"/>
<title>{}</title>
</head>
<body>
<div>
<h3>{}</h3>"#,
            $title, $title
        )
    }
}
#[macro_export]
macro_rules! chapter_line {
    ($line: expr) => {
        format_args!(r"
<p>{}</p>", $line)
    }
}

pub const CHAPTER_END: &'static str = r"
</div>
</body>
</html>";


pub mod patterns {
    pub const DEFAULT_VOLUME_PAT: &'static str = 
        r"^\s*第[零一二两三四五六七八九十百千万0-9]{1,4}卷[　 ]+(?<vol_name>\S*)\s*$";

    pub const DEFAULT_CHAPTER_PAT: &'static str = 
        r"^\s*第[零一二两三四五六七八九十百千万0-9]{1,7}章[　 ]+(?<chap_name>\S*)\s*$";

    pub const DEFAULT_LINE_PAT: &'static str =
        r"^[\s　]*(?<line>[^\s　]+)[\s　]*$";
}
