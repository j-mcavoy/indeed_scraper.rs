use chrono::{DateTime, Utc};
use url::{ParseError, Url};

extern crate strum;
use strum::AsStaticRef;

#[allow(non_camel_case_types)]
#[derive(AsStaticStr, Clone, Debug)]
pub enum Sort {
    relevance,
    date,
}

#[allow(non_camel_case_types)]
#[derive(AsStaticStr, Clone, Debug)]
pub enum ShowJobsFrom {
    all,
    jobsite,
    employer,
}

#[allow(non_camel_case_types)]
#[derive(AsStaticStr, Clone, Debug)]
pub enum JobType {
    fulltime,
    contract,
    parttime,
    temporary,
    internship,
    commission,
}

#[allow(non_camel_case_types)]
#[derive(AsStaticStr, Clone, Debug)]
pub enum Level {
    entry_level,
    mid_level,
    senior_level,
}

#[derive(Debug, Clone)]
pub struct Company {
    name: String,
    location: String,
    homepage: Url,
    rating: f32,
}

#[derive(Debug, Clone)]
pub struct Job {
    job_title: String,
    location: String,
    annual_salary: Option<f32>,
    date_posted: DateTime<Utc>,
    url: Url,
}

#[derive(Debug, Clone)]
pub enum Salary {
    AnnualFixed(u32),
    AnnualRange(u32, u32),
    HourlyFixed(f32),
    HourlyRange(f32),
}

#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
#[builder(setter(into))]
pub struct IndeedQuery<'a> {
    #[builder(default = "0")]
    radius: u32,
    city: String,
    level: Level,
    #[builder(default = "14")]
    max_age: u32,
    #[builder(default = "Sort::relevance")]
    sort: Sort,
    #[builder(default = "JobType::fulltime")]
    job_type: JobType,
    #[builder(default = "0")]
    min_salary: u32,
    #[builder(default = "Vec::new()")]
    contains_all_words: Vec<&'a str>,
    #[builder(default = "\"\"")]
    has_exact_phrase: &'a str,
    #[builder(default = "Vec::new()")]
    has_any_words: Vec<&'a str>,
    #[builder(default = "Vec::new()")]
    excludes_all_words: Vec<&'a str>,
    #[builder(default = "Vec::new()")]
    has_words_in_title: Vec<&'a str>,
    #[builder(default = "\"\"")]
    company: &'a str,
    #[builder(default = "true")]
    exclude_staffing_agencies: bool,
    #[builder(default = "ShowJobsFrom::all")]
    show_jobs_from: ShowJobsFrom,
    #[builder(default = "50")]
    limit: u32,
    #[builder(default = "0")]
    start: u32,
}
impl IndeedQuery<'_> {
    pub fn build_url(&self) -> Result<Url, ParseError> {
        let url = Url::parse_with_params(
            "https://www.indeed.com/jobs",
            &[
                ("as_and", self.contains_all_words.join(" ")),
                ("as_phr", self.has_exact_phrase.to_string()),
                ("as_any", self.has_any_words.join(" ")),
                ("as_not", self.excludes_all_words.join(" ")),
                ("as_ttl", self.has_words_in_title.join(" ")),
                ("as_cmp", self.company.to_string()),
                ("jt", self.job_type.as_static().to_string()),
                ("st", self.show_jobs_from.as_static().to_string()),
                (
                    "sr",
                    if self.exclude_staffing_agencies {
                        "directhire".to_string()
                    } else {
                        "".to_string()
                    },
                ),
                ("salary", self.min_salary.to_string()),
                ("radius", self.radius.to_string()),
                ("l", self.city.to_string()),
                ("fromage", self.max_age.to_string()),
                ("limit", self.limit.to_string()),
                ("sort", self.sort.as_static().to_string()),
                ("psf", "advsrch".to_string()),
                ("from", "advancedsearch".to_string()),
            ],
        )?;
        Ok(url)
    }

    pub fn increment_page(&self) {
        self.start += self.limit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_url_test() {
        let query = IndeedQueryBuilder::default()
            .has_words_in_title(vec!["Developer", "Engineer"])
            .excludes_all_words(vec!["C#", ".NET", "Azure", "Unpaid"])
            .exclude_staffing_agencies(true)
            .has_any_words(vec![
                "Linux",
                "Unix",
                "Full-Stack",
                "Full",
                "Stack",
                "Web",
                "Embedded",
            ])
            .min_salary(85000 as u32)
            .city("San Fransisco, CA")
            .radius(40 as u32)
            .job_type(JobType::fulltime)
            .level(Level::entry_level)
            .sort(Sort::date)
            .max_age(14 as u32)
            .build()
            .unwrap();
        assert_eq!(query.build_url(), Url::parse("https://www.indeed.com/jobs?as_and=&as_phr=&as_any=Linux+Unix+Full-Stack+Full+Stack+Web+Embedded&as_not=C%23+.NET+Azure+Unpaid&as_ttl=Developer+Engineer&as_cmp=&jt=fulltime&st=all&sr=directhire&salary=85000&radius=40&l=San+Fransisco%2C+CA&fromage=14&limit=50&sort=date&psf=advsrch&from=advancedsearch"));
    }
}
