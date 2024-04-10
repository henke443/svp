use std::env;
use std::path::PathBuf;
use std::io::{Read, Seek, Write};
use regex::{Captures, Regex, Replacer};


extern "C" {
    fn testing(x: u8) -> u8;
}


struct SvgSimplifier;

impl Replacer for SvgSimplifier {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        dst.push_str(&caps["first"]);
        dst.push_str(" ");
        dst.push_str(&caps["last"]);
    }
}


pub fn svg_replacer(svg_data: &str, i: u32) -> String {
    let re = Regex::new(r##"((xlink:href="#\w+)|(id="\w+)|(url\(#\w+))"##).unwrap();
    let result = re.replace_all(svg_data, format!("${{1}}--{}", i)).to_string()
        // Make glyphs work when concatenated
        .replace("url(#clip" , &format!("url(#clip-{}", i))
        .replace("#glyph" , &format!("#glyph-{}", i))
        .replace("id=\"clip" , &format!("id=\"clip-{}", i))
        .replace("id=\"glyph" , &format!("id=\"glyph-{}", i))
        // Remove svg bloat
        .replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?>" , "")
        .replace("xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" ", "");
    result
}

// Concatenate svg files in the array into a .svp file.
pub fn make_svp(files: &[&str], output: &str) {
    let mut svp = std::fs::File::create(output).unwrap();
    svp.write_all(b"<svp version=\"0.1.0\">\n").unwrap();
    let mut i = 0u32;
    for file in files {
        i += 1;
        let mut svg = std::fs::File::open(file).unwrap();
        let mut buffer = Vec::new();
        svg.read_to_end(&mut buffer).unwrap();

        let mut svg_data = String::from_utf8(buffer.clone()).unwrap();
        let final_svg = svg_replacer(&svg_data, i);
        buffer = final_svg.into_bytes();

        svp.write_all(&buffer).unwrap();
    }
    svp.write_all(b"\n</svp>").unwrap();
}

pub fn make_svp_html(svp_content: &str, output_path: &str) {
    let mut svp_html = std::fs::File::create(output_path).unwrap();
    svp_html.write_all(br#"<html>
<head>
<title>SVP Document</title>
<style>
body, html {
    width: 100%;
    background: #333;

    height: 100%;
}
svg {  
    display: block;
    background: white;
    box-sizing: border-box;
    padding: 2rem;
    margin: 2rem auto;
    height: 100vh;
    width: auto;
}
</style>
</head>
<body>"#).unwrap();
    svp_html.write_all(svp_content.as_bytes()).unwrap();
    svp_html.write_all(b"\n</body>\n</html>").unwrap();
}

pub fn pdf_to_svg(input: &str, output: &str) {
    let output = std::process::Command::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pdf2svg-windows/dist-64bits/pdf2svg.exe").to_str().unwrap())
        .arg(input)
        .arg(output)
        .arg("all")
        .output()
        .expect("failed to execute process");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = unsafe { testing(21) };
        assert_eq!(a, 42);
    }

    #[test]
    fn it_really_works() {
        pdf_to_svg("test.pdf", "test_output/output_%d.svg");
        let mut files: Vec<String> = vec![]; // Change the type of `files` to `Vec<String>`
        for i in 1..14 {
            let svg_path = format!("test_output/output_{}.svg", i);
            assert!(std::path::Path::new(svg_path.as_str()).exists());
            files.push(svg_path); // Push `svg_path` as a `String`
        }
        let str_files = &files.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        make_svp(str_files, "test_output/output.svp"); // Convert `Vec<String>` to `&[&str]`
        let svp_contents = std::fs::read_to_string("test_output/output.svp").unwrap();
        make_svp_html(svp_contents.as_str(), "test_output/output.svp.html");
        //
    }
}
