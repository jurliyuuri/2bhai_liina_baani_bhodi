#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    MismatchedRightLenticular(String),
    MismatchedLeftLenticular(String),
    LenticularInsideLenticular(String),
}

pub trait Lenticular
where
    Self: std::marker::Sized,
{
    fn lenticular_to_link(&self) -> Result<Self, Vec<Error>>;
}

impl<T, U> Lenticular for (T, U)
where
    T: Lenticular + Clone,
    U: Lenticular + Clone,
{
    fn lenticular_to_link(&self) -> Result<Self, Vec<Error>> {
        let (a, b) = self;
        match (a.lenticular_to_link(), b.lenticular_to_link()) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            (Ok(_), Err(b)) | (Err(b), Ok(_)) => Err(b),
            (Err(mut a), Err(mut b)) => {
                a.append(&mut b);
                Err(a)
            }
        }
    }
}

impl<T> Lenticular for Vec<T>
where
    T: Lenticular + Clone,
{
    fn lenticular_to_link(&self) -> Result<Self, Vec<Error>> {
        let mut ans = Vec::new();
        let mut errors = Vec::new();
        for i in self.iter() {
            match i.clone().lenticular_to_link() {
                Ok(l) => ans.push(l),
                Err(mut e) => errors.append(&mut e),
            }
        }

        if errors.is_empty() {
            Ok(ans)
        } else {
            Err(errors)
        }
    }
}

impl Lenticular for String {
    /// Convert lenticular bracket into a link
    fn lenticular_to_link(&self) -> Result<Self, Vec<Error>> {
        let mut inside_lenticular = false;
        let mut ans = String::new();
        for c in self.chars() {
            if inside_lenticular {
                match c {
                    '【' => {
                        return Err(vec![Error::LenticularInsideLenticular(
                            self.clone(),
                        )])
                    }
                    '】' => {
                        inside_lenticular = false;
                    }
                    linzi => {
                        ans += &format!(
                            "<a href=\"{linzi}%20-%20燐字海.html\">{linzi}</a>",
                            linzi = linzi
                        );
                    }
                }
            } else {
                match c {
                    '【' => {
                        inside_lenticular = true;
                    }
                    '】' => {
                        return Err(vec![Error::MismatchedRightLenticular(
                            self.clone(),
                        )])
                    }
                    linzi => ans.push(linzi),
                }
            }
        }

        if inside_lenticular {
            return Err(vec![Error::MismatchedLeftLenticular(
                self.clone(),
            )]);
        }

        Ok(ans)
    }
}
