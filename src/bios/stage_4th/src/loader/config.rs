#[derive(Debug)]
pub struct Config<'a> {
    pub kernel: &'a str,
    pub initrd: Option<&'a str>,
    pub cmdlin: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn new(contents: &'a str) -> Result<Self, &'static str> {
        let kernel = get_item(contents, "main.kernel");
        let initrd = get_item(contents, "main.initrd");
        let cmdlin = get_item(contents, "main.cmdlin");

        if let Some(kernel) = kernel {
            Ok(Self {
                kernel,
                initrd,
                cmdlin,
            })
        } else {
            Err("Kernel file is not specifyed!")
        }
    }
}

fn get_item<'a, 'b>(contents: &'a str, pat: &'b str) -> Option<&'a str> {
    let mut res = None;
    for line in contents.lines() {
        if line.starts_with(pat) {
            res = Some(line.split_at(pat.len() + 1).1);
            break;
        }
    }
    res
}
