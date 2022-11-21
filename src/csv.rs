use csv::Reader;
use std::error::Error;

/// Convert csv content to target format.
/// 
/// # Example
/// 
/// ```
/// let res: &str = &convert("a,b\n1,2", "html")
/// assert_eq!(res, "<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody>\
///     <tr><td>1</td><td>2</td></tr></tbody></table>")
/// ```
pub fn convert(contents: &str, target: &str) -> Result<String, Box<dyn Error>> {
    let rdr = Reader::from_reader(contents.as_bytes());

    match target {
        "html" => to_html(rdr),
        "md" => to_markdown(rdr),
        _ => Err("unsupported target extension".into()),
    }
}

/// Convert csv to html.
pub fn to_html(mut rdr: Reader<&[u8]>) -> Result<String, Box<dyn Error>> {
    let headers = rdr.headers()?.clone();
    let records = rdr.records();
    let mut res = String::new();
    res += "<table><thead><tr>";
    for header in &headers {
        let new: &str = &format!("<th>{}</th>", header);
        res += new;
    }
    res += "</tr></thead><tbody>";
    for result in records {
        let record = result?;
        res += "<tr>";
        for cell in &record {
            let new: &str = &format!("<td>{}</td>", cell);
            res += new;
        }
        res += "</tr>"
    }
    res += "</tbody></table>";
    Ok(res)
}

/// Convert csv to markdown.
pub fn to_markdown(mut rdr: Reader<&[u8]>) -> Result<String, Box<dyn Error>> {
    let headers = rdr.headers()?.clone();
    let records = rdr.records();
    let mut res = String::new();
    res += "|";
    for header in &headers {
        let new: &str = &format!("{}|", header);
        res += new;
    }
    res += "\n|";
    for _ in 0..headers.len() {
        res += "---|"
    }
    for result in records {
        let record = result?;
        res += "\n|";
        for cell in &record {
            let new: &str = &format!("{}|", cell);
            res += new;
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_html() {
        assert_eq!(
            convert("a,total\n3,B", "html").unwrap(),
            "<table><thead><tr><th>a</th><th>total</th></tr></thead><tbody>\
            <tr><td>3</td><td>B</td></tr></tbody></table>"
        );
    }

    #[test]
    fn to_markdown() {
        assert_eq!(
            convert("a,total\n3,B", "md").unwrap(),
            "|a|total|\n|---|---|\n|3|B|"
        );
    }

    #[test]
    fn unsupported_target_extension() {
        assert!(convert("a,total\n3,B\n9,E", "cc").is_err());
    }
}
