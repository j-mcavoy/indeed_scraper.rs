use super::super::*;
use url::Url;

use quick_crawler::{
    QuickCrawler,
    QuickCrawlerBuilder,
    limiter::Limiter,
    RequestHandlerConfig,
    QuickCrawlerError,
    scrape::{
        ResponseLogic::Parallel,
        StartUrl,
        Scrape,
        ElementUrlExtractor,
        ElementDataExtractor
    }
};

enum Sort { Relevance, Date }

enum JobType { Fulltime, Contract, Parttime, Temporary, Internship, Commission }

pub struct IndeedQuery<'a> {
    job_title: &'a str,
    location: Location,
    radius: u16,
    city: &'a str,
    level: &'a str,
    max_age: u16,
    sort: Sort,
    job_type: JobType,
    excludeSponsored: bool,
}

impl super::Scrape for IndeedQuery {
    fn build_url(&self) -> Url {
        Url::parse("https://www.indeed.com/jobs").unwrap()
        .join("?q=").unwrap()
        .join(self.job_title).unwrap()
        .join("?l=").unwrap()
        .join(self.city).unwrap()
        .join("?radius=").unwrap()
        .join(self.radius.to_string().as_str()).unwrap()
        .join("?fromage=").unwrap()
        .join(self.max_age.to_string().as_str()).unwrap()
    }
    fn scrape(&self) -> (Company, Job) {

        let mut builder = QuickCrawlerBuilder::new();

        let start_urls = vec![
            StartUrl::new()
                .url("https://indeed")
                .method("GET")
                .response_logic(Parallel(vec![
                        // All Scrapers below will be provided the html page response body
                        Scrape::new()
                        .find_elements_with_urls(".bike-item")
                        .extract_urls_from_elements(ElementUrlExtractor::Attr("href".to_string()))
                        // now setup the logic to execute on each of the return html pages
                        .response_logic(Parallel(vec![
                                Scrape::new()
                                .find_elements_with_data(".awesome-bike .bike-info")
                                .extract_data_from_elements(ElementDataExtractor::Text)
                                .store(|vec: Vec<String>| async move {
                                    println!("store bike info in DB: {:?}", vec);
                                }),
                                Scrape::new()
                                .find_elements_with_data(".bike-specs li")
                                .extract_data_from_elements(ElementDataExtractor::Text)
                                .store(|vec: Vec<String>| async move {
                                    println!("store bike specs in DB: {:?}", vec);
                                }),
                        ])),
                        Scrape::new()
                            .find_elements_with_urls(".bike-other-item")
                            .extract_urls_from_elements(ElementUrlExtractor::Attr("href".to_string()))
                            .response_logic(Parallel(vec![
                                    Scrape::new()
                                    .find_elements_with_data(".other-bike .other-bike-info")
                                    .extract_data_from_elements(ElementDataExtractor::Text)
                                    .store(|vec: Vec<String>| async move {
                                        println!("store other bike info in DB: {:?}", vec);
                                    }),
                                    Scrape::new()
                                    .find_elements_with_data(".other-bike-specs li")
                                    .extract_data_from_elements(ElementDataExtractor::Text)
                                    .store(|vec: Vec<String>| async move {
                                        println!("store other bike specs in DB: {:?}", vec);
                                    }),
                            ]))
                            ])
                            )
                            // more StartUrl::new 's if you feel ambitious
                            ] ;

        // It's smart to use a limiter - for now automatically set to 3 request per second per domain.
        // This will soon be configurable.

        let limiter = Limiter::new();

        builder
            .with_start_urls(
                start_urls
            )
            .with_limiter(
                limiter
            )
            // Optionally configure how to make a request and return an html string
            .with_request_handler(
                |config: RequestHandlerConfig| async move {
                    // ... use any request library, like reqwest
                    surf::get(config.url.clone()).recv_string().await.map_err(|_| QuickCrawlerError::RequestErr)
                            }
            );

        let crawler = builder.finish().map_err(|_| "Builder could not finish").expect("no error");

        // QuickCrawler is async, so choose your favorite executor.
        // (Tested and working for both async-std and tokio)
        let res = async_std::task::block_on(async {
            crawler.process().await
        });

        (Company {Location::new("Sanf")}, Job {})
    }
}
