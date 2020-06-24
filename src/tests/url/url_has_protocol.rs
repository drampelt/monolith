//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod passing {
    use crate::url;

    #[test]
    fn mailto() {
        assert!(url::url_has_protocol(
            "mailto:somebody@somewhere.com?subject=hello"
        ));
    }

    #[test]
    fn tel() {
        assert!(url::url_has_protocol("tel:5551234567"));
    }

    #[test]
    fn ftp_no_slashes() {
        assert!(url::url_has_protocol("ftp:some-ftp-server.com"));
    }

    #[test]
    fn ftp_with_credentials() {
        assert!(url::url_has_protocol(
            "ftp://user:password@some-ftp-server.com"
        ));
    }

    #[test]
    fn javascript() {
        assert!(url::url_has_protocol("javascript:void(0)"));
    }

    #[test]
    fn http() {
        assert!(url::url_has_protocol("http://news.ycombinator.com"));
    }

    #[test]
    fn https() {
        assert!(url::url_has_protocol("https://github.com"));
    }

    #[test]
    fn mailto_uppercase() {
        assert!(url::url_has_protocol(
            "MAILTO:somebody@somewhere.com?subject=hello"
        ));
    }
}

//  ███████╗ █████╗ ██╗██╗     ██╗███╗   ██╗ ██████╗
//  ██╔════╝██╔══██╗██║██║     ██║████╗  ██║██╔════╝
//  █████╗  ███████║██║██║     ██║██╔██╗ ██║██║  ███╗
//  ██╔══╝  ██╔══██║██║██║     ██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║██║███████╗██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod failing {
    use crate::utils;

    #[test]
    fn url_with_no_protocol() {
        assert!(!url::url_has_protocol(
            "//some-hostname.com/some-file.html"
        ));
    }

    #[test]
    fn relative_path() {
        assert!(!url::url_has_protocol("some-hostname.com/some-file.html"));
    }

    #[test]
    fn relative_to_root_path() {
        assert!(!url::url_has_protocol("/some-file.html"));
    }

    #[test]
    fn empty_string() {
        assert!(!url::url_has_protocol(""));
    }
}