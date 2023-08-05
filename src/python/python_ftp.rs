pub enum PythonFTP {
    Python3_5_0(String),
    Python3_5_1(String),
    Python3_5_2(String),
    Python3_5_3(String),
    Python3_5_4(String),
    Python3_6_0(String),
    Python3_6_1(String),
    Python3_6_2(String),
    Python3_6_3(String),
    Python3_6_4(String),
    Python3_6_5(String),
    Python3_6_6(String),
    Python3_6_7(String),
    Python3_6_8(String),

    Python3_7_0(String),
    Python3_7_1(String),
    Python3_7_2(String),
    Python3_7_3(String),
    Python3_7_4(String),
    Python3_7_5(String),
    Python3_7_6(String),
    Python3_7_7(String),
    Python3_7_8(String),
    Python3_7_9(String),
}

impl PythonFTP {
    pub fn from_tuple(version: (usize, usize, usize)) -> Option<Self> {
        match version {
            (3, 5, 0) => Some(PythonFTP::Python3_5_0(
                "https://www.python.org/ftp/python/3.5.0/python-3.5.0-amd64.exe".to_string(),
            )),

            (3, 5, 1) => Some(PythonFTP::Python3_5_1(
                "https://www.python.org/ftp/python/3.5.1/python-3.5.1-amd64.exe".to_string(),
            )),

            (3, 5, 2) => Some(PythonFTP::Python3_5_2(
                "https://www.python.org/ftp/python/3.5.2/python-3.5.2-amd64.exe".to_string(),
            )),

            (3, 5, 3) => Some(PythonFTP::Python3_5_3(
                "https://www.python.org/ftp/python/3.5.3/python-3.5.3-amd64.exe".to_string(),
            )),

            (3, 5, 4) => Some(PythonFTP::Python3_5_4(
                "https://www.python.org/ftp/python/3.5.4/python-3.5.4-amd64.exe".to_string(),
            )),

            (3, 6, 0) => Some(PythonFTP::Python3_6_0(
                "https://www.python.org/ftp/python/3.6.0/python-3.6.0-amd64.exe".to_string(),
            )),

            (3, 6, 1) => Some(PythonFTP::Python3_6_1(
                "https://www.python.org/ftp/python/3.6.1/python-3.6.1-amd64.exe".to_string(),
            )),

            (3, 6, 2) => Some(PythonFTP::Python3_6_2(
                "https://www.python.org/ftp/python/3.6.2/python-3.6.2-amd64.exe".to_string(),
            )),

            (3, 6, 3) => Some(PythonFTP::Python3_6_3(
                "https://www.python.org/ftp/python/3.6.3/python-3.6.3-amd64.exe".to_string(),
            )),

            (3, 6, 4) => Some(PythonFTP::Python3_6_4(
                "https://www.python.org/ftp/python/3.6.4/python-3.6.4-amd64.exe".to_string(),
            )),

            (3, 6, 5) => Some(PythonFTP::Python3_6_5(
                "https://www.python.org/ftp/python/3.6.5/python-3.6.5-amd64.exe".to_string(),
            )),

            (3, 6, 6) => Some(PythonFTP::Python3_6_6(
                "https://www.python.org/ftp/python/3.6.6/python-3.6.6-amd64.exe".to_string(),
            )),

            (3, 6, 7) => Some(PythonFTP::Python3_6_7(
                "https://www.python.org/ftp/python/3.6.7/python-3.6.7-amd64.exe".to_string(),
            )),

            (3, 6, 8) => Some(PythonFTP::Python3_6_8(
                "https://www.python.org/ftp/python/3.6.8/python-3.6.8-amd64.exe".to_string(),
            )),

            (3, 7, 0) => Some(PythonFTP::Python3_7_0(
                "https://www.python.org/ftp/python/3.7.0/python-3.7.0-amd64.exe".to_string(),
            )),

            (3, 7, 1) => Some(PythonFTP::Python3_7_1(
                "https://www.python.org/ftp/python/3.7.1/python-3.7.1-amd64.exe".to_string(),
            )),

            (3, 7, 2) => Some(PythonFTP::Python3_7_2(
                "https://www.python.org/ftp/python/3.7.2/python-3.7.2-amd64.exe".to_string(),
            )),

            (3, 7, 3) => Some(PythonFTP::Python3_7_3(
                "https://www.python.org/ftp/python/3.7.3/python-3.7.3-amd64.exe".to_string(),
            )),

            (3, 7, 4) => Some(PythonFTP::Python3_7_4(
                "https://www.python.org/ftp/python/3.7.4/python-3.7.4-amd64.exe".to_string(),
            )),

            (3, 7, 5) => Some(PythonFTP::Python3_7_5(
                "https://www.python.org/ftp/python/3.7.5/python-3.7.5-amd64.exe".to_string(),
            )),

            (3, 7, 6) => Some(PythonFTP::Python3_7_6(
                "https://www.python.org/ftp/python/3.7.6/python-3.7.6-amd64.exe".to_string(),
            )),

            (3, 7, 7) => Some(PythonFTP::Python3_7_7(
                "https://www.python.org/ftp/python/3.7.7/python-3.7.7-amd64.exe".to_string(),
            )),

            (3, 7, 8) => Some(PythonFTP::Python3_7_8(
                "https://www.python.org/ftp/python/3.7.8/python-3.7.8-amd64.exe".to_string(),
            )),

            (3, 7, 9) => Some(PythonFTP::Python3_7_9(
                "https://www.python.org/ftp/python/3.7.9/python-3.7.9-amd64.exe".to_string(),
            )),

            _ => None,
        }
    }

    pub fn get_url(&self) -> &str {
        match self {
            PythonFTP::Python3_5_0(url) => url,
            PythonFTP::Python3_5_1(url) => url,
            PythonFTP::Python3_5_2(url) => url,
            PythonFTP::Python3_5_3(url) => url,
            PythonFTP::Python3_5_4(url) => url,
            PythonFTP::Python3_6_0(url) => url,
            PythonFTP::Python3_6_1(url) => url,
            PythonFTP::Python3_6_2(url) => url,
            PythonFTP::Python3_6_3(url) => url,
            PythonFTP::Python3_6_4(url) => url,
            PythonFTP::Python3_6_5(url) => url,
            PythonFTP::Python3_6_6(url) => url,
            PythonFTP::Python3_6_7(url) => url,
            PythonFTP::Python3_6_8(url) => url,
        }
    }
}
