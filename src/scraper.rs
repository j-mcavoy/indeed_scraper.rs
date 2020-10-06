use crate::query_builder::*;
use scraper::{Element, ElementRef, Html, Selector};
use std::collections::HashMap;
use ureq::get;

pub fn scrape(query: IndeedQuery<'_>) -> HashMap<Company, Job> {
    let mut map = HashMap::new();
    let mut jobs: Vec<Job> = vec![];

    let resp = get(query.build_url().unwrap().as_str()).call();
    // TODO ensure resp.ok()
    let raw_first_page = resp.into_string().unwrap();
    let first_page = Html::parse_fragment(raw_first_page.as_str());
    let (mut curr_page_num, pages) = parse_current_page(&first_page);

    // TODO make async
    while curr_page_num <= pages {
        let resp = get(query.build_url().unwrap().as_str()).call();
        let raw_page = resp.into_string().unwrap();
        let curr_page = Html::parse_fragment(raw_page.as_str());
        let jobs_on_page: Vec<Job> = parse_jobs(&curr_page);
        curr_page_num += 1;
    }
    map
}

// TODO: error handling
fn parse_jobs(page: &Html) -> Vec<Job> {
    let jobs = vec![];
    let selector = Selector::parse("a.jobtitle").unwrap();
    // TODO: make multi threaded & async
    for job_el in page.select(&selector) {
        let job: Job = crawl_job_details(job_el);
        jobs.push(job);
    }

    jobs
}

fn crawl_job_details(el: Element) -> Job {
    let job_title_selector = Selector::parse(".jobtitle");
    let company_selector = Selector::parse(".company");
    let location_selector = Selector::parse(".location");
    let salary_selector = Selector::parse(".salaryText");

    let job_title: String = el.select(&job_title_selector).next();

    let url = el.attr("href");

    let resp = get(job_url).call();
    let raw_job = resp.into_string().unwrap();
    let job_html = Html::parse_fragment(raw_page.as_str());

    let  = Selector::parse(".jobtitle");

    Job {}
}

fn parse_current_page(first_page: &Html) -> (u32, u32) {
    let pagesSelector = Selector::parse("#searchCountPages").unwrap();
    let el: scraper::ElementRef = first_page.select(&pagesSelector).next().unwrap();
    let s = el.inner_html();
    let v: Vec<&str> = s.split(' ').collect();
    let current_page = v[v.len() - 4].parse::<u32>().unwrap();
    let pages = v[v.len() - 2].parse::<u32>().unwrap();
    (current_page, pages)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scrape_test() {
        let mut query = IndeedQueryBuilder::default()
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
        scrape(query);
    }
}
