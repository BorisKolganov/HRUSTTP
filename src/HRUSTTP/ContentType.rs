pub fn lookup(extension: &str) -> String {
        match extension {
            "html" => "text/html".into(),
            "jpeg" => "image/jpeg".into(),
            "css" => "text/css".into(),
            "jpg" => "image/jpeg".into(),
            "js" => "application/javascript".into(),
            "png" => "image/png".into(),
            "gif" => "image/gif".into(),
            "swf" => "application/x-shockwave-flash".into(),
            _ => "plain/text".into(),
        }
}
