// Date:   Wed Jul 03 12:32:52 2024
// Mail:   lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

pub mod common;
pub mod utils;
pub mod err;

#[cfg(test)]
mod tests {
    #[test]
    pub fn regex_test() {
        use regex::Regex;
        const SOURCE: &'static str = r"第一卷   夜游神   
            ";
        let re = Regex::new(r"第[零一二两三四五六七八九十百千万0-9\.]{1,4}卷\s*(?<vol_name>\S*)\s*$")
            .unwrap();
        let vol_caps = re.captures(SOURCE).unwrap();
        let vol_name = &vol_caps["vol_name"];
        println!("<{}>", vol_name);
    }

    #[test]
    pub fn regexset_test() {
        use regex::{Regex, Captures};
        const SOURCE: &[&'static str] = &[
            r"第一卷 夜游神",
            r"第一回 夜游神",
            r"第一本 夜游神"
        ];

        const PATS: &[&'static str] = &[
            r"第[零一二两三四五六七八九十百千万0-9]{1,4}卷\s*(?<vol_name>\S*)\s*$",
            r"第[零一二两三四五六七八九十百千万0-9]{1,4}[回本]\s*(?<vol_name>\S*)\s*$",
        ];

        let re_set = PATS.iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect::<Vec<Regex>>();

        SOURCE.iter()
            .for_each(|src| {
                let cap = re_set.iter()
                    .find_map(|re| re.captures(src));
                if let Some(cap) = cap {
                    // println!("Capture: <{}>", &cap["vol_name"]);
                    assert_eq!("夜游神", &cap["vol_name"]);
                }
            })
    }

    #[test]
    pub fn regex_test2() {
        use regex::Regex;
        const PAT: &'static str = r"^[\s　]*(?<line>[^\s　]+)[\s　]*$";
        let re = Regex::new(PAT).unwrap();
        [
            (r"　　 全角空格开始 ", "全角空格开始")
        ].iter()
        .for_each(|(src, expect)| {
            let cap = re.captures(src).unwrap();
            assert_eq!(&cap["line"], *expect);
        });
    }

    #[test]
    pub fn path_test() {
        use std::path::Path;
        assert_eq!("Gone", Path::new("txt/Gone.txt").file_stem().unwrap());
    }
}
