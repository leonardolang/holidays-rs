
//! Malaysia
use super::*;

/// Generate holiday map for Malaysia.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
  let mut map = HashMap::new();

  build_year(
    years,
    2000,
    vec![

      (NaiveDate::from_ymd_res(2000, 2, 5)?, "Chinese New Year (estimated)"),
      (NaiveDate::from_ymd_res(2000, 2, 6)?, "Chinese New Year Holiday (estimated)"),
      (NaiveDate::from_ymd_res(2000, 5, 18)?, "Vesak Day (estimated)"),
      (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2000, 6, 3)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2000, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2000, 10, 25)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2000, 6, 14)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2000, 1, 8)?, "Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2000, 12, 27)?, "Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2000, 1, 9)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2000, 12, 28)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2000, 3, 16)?, "Hari Raya Haji (estimated)"),
      (NaiveDate::from_ymd_res(2000, 1, 10)?, "Second day of Hari Raya Puasa (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2000, 2, 7)?, "Chinese New Year Holiday (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2000, 4, 6)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2001,
    vec![

      (NaiveDate::from_ymd_res(2001, 1, 24)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2001, 1, 25)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2001, 5, 7)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2001, 6, 2)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2001, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2001, 11, 14)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2001, 6, 4)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2001, 12, 17)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2001, 12, 18)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2001, 3, 6)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2001, 3, 26)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2002,
    vec![

      (NaiveDate::from_ymd_res(2002, 2, 12)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2002, 2, 13)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2002, 5, 27)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2002, 6, 1)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2002, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2002, 11, 3)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2002, 5, 24)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2002, 12, 6)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2002, 12, 7)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2002, 2, 23)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2002, 11, 4)?, "Deepavali (in lieu)"),
      (NaiveDate::from_ymd_res(2002, 3, 15)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2003,
    vec![

      (NaiveDate::from_ymd_res(2003, 2, 1)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2003, 2, 2)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2003, 5, 15)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2003, 6, 7)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2003, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2003, 10, 23)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2003, 5, 14)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2003, 11, 26)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2003, 11, 27)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2003, 2, 12)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2003, 2, 3)?, "Chinese New Year Holiday (in lieu)"),
      (NaiveDate::from_ymd_res(2003, 9, 1)?, "National Day (in lieu)"),
      (NaiveDate::from_ymd_res(2003, 3, 5)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2004,
    vec![

      (NaiveDate::from_ymd_res(2004, 1, 22)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2004, 1, 23)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2004, 5, 3)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2004, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2004, 6, 5)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2004, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2004, 11, 11)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2004, 5, 2)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2004, 11, 14)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2004, 11, 15)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2004, 2, 2)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2004, 5, 4)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (in lieu)"),
      (NaiveDate::from_ymd_res(2004, 11, 16)?, "Hari Raya Puasa (in lieu)"),
      (NaiveDate::from_ymd_res(2004, 2, 22)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2005,
    vec![

      (NaiveDate::from_ymd_res(2005, 2, 9)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2005, 2, 10)?, "Awal Muharram (Hijri New Year); Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2005, 5, 22)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2005, 6, 4)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2005, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2005, 11, 1)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2005, 4, 21)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2005, 11, 3)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2005, 11, 4)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2005, 1, 21)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2005, 5, 2)?, "Labour Day (in lieu)"),
      (NaiveDate::from_ymd_res(2005, 5, 23)?, "Vesak Day (in lieu)"),
      (NaiveDate::from_ymd_res(2005, 12, 26)?, "Christmas Day (in lieu)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2006,
    vec![

      (NaiveDate::from_ymd_res(2006, 1, 29)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2006, 1, 30)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2006, 5, 12)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2006, 6, 3)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2006, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2006, 10, 21)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2006, 4, 11)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2006, 10, 24)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2006, 10, 25)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2006, 1, 10)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2006, 12, 31)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2006, 1, 31)?, "Awal Muharram (Hijri New Year); Chinese New Year (in lieu)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2007,
    vec![

      (NaiveDate::from_ymd_res(2007, 2, 18)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2007, 2, 19)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day; Vesak Day"),
      (NaiveDate::from_ymd_res(2007, 6, 2)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2007, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2007, 11, 8)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2007, 3, 31)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2007, 10, 13)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2007, 10, 14)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2007, 12, 20)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2007, 2, 20)?, "Chinese New Year (in lieu)"),
      (NaiveDate::from_ymd_res(2007, 10, 15)?, "Second day of Hari Raya Puasa (in lieu)"),
      (NaiveDate::from_ymd_res(2007, 1, 2)?, "Hari Raya Haji (in lieu)"),
      (NaiveDate::from_ymd_res(2007, 1, 20)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2008,
    vec![

      (NaiveDate::from_ymd_res(2008, 2, 7)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2008, 2, 8)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2008, 5, 19)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2008, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2008, 6, 7)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2008, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2008, 10, 27)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2008, 3, 20)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2008, 10, 1)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2008, 10, 2)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2008, 12, 9)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2008, 9, 1)?, "National Day (in lieu)"),
      (NaiveDate::from_ymd_res(2008, 1, 10)?, "Awal Muharram (Hijri New Year)"),
      (NaiveDate::from_ymd_res(2008, 12, 29)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2009,
    vec![

      (NaiveDate::from_ymd_res(2009, 1, 26)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2009, 1, 27)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2009, 5, 9)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2009, 6, 6)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2009, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2009, 10, 17)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2009, 3, 9)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2009, 9, 20)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2009, 9, 21)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2009, 11, 28)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2009, 9, 22)?, "Hari Raya Puasa (in lieu)"),
      (NaiveDate::from_ymd_res(2009, 12, 18)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2010,
    vec![

      (NaiveDate::from_ymd_res(2010, 2, 14)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2010, 2, 15)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2010, 5, 28)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2010, 6, 5)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2010, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2010, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2010, 11, 5)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2010, 2, 26)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2010, 9, 10)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2010, 9, 11)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2010, 11, 17)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2010, 2, 16)?, "Chinese New Year (in lieu)"),
      (NaiveDate::from_ymd_res(2010, 12, 8)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2011,
    vec![

      (NaiveDate::from_ymd_res(2011, 2, 3)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2011, 2, 4)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2011, 5, 17)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2011, 6, 4)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2011, 8, 31)?, "Hari Raya Puasa; National Day"),
      (NaiveDate::from_ymd_res(2011, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2011, 10, 26)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2011, 2, 16)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2011, 9, 1)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2011, 11, 7)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2011, 5, 2)?, "Labour Day (in lieu)"),
      (NaiveDate::from_ymd_res(2011, 12, 26)?, "Christmas Day (in lieu)"),
      (NaiveDate::from_ymd_res(2011, 11, 27)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2012,
    vec![

      (NaiveDate::from_ymd_res(2012, 1, 23)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2012, 1, 24)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2012, 5, 5)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2012, 6, 2)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2012, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2012, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2012, 11, 13)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2012, 2, 5)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2012, 8, 19)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2012, 8, 20)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2012, 10, 26)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2012, 2, 6)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (in lieu)"),
      (NaiveDate::from_ymd_res(2012, 8, 21)?, "Hari Raya Puasa (in lieu)"),
      (NaiveDate::from_ymd_res(2012, 9, 17)?, "Malaysia Day (in lieu)"),
      (NaiveDate::from_ymd_res(2012, 11, 15)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2013,
    vec![

      (NaiveDate::from_ymd_res(2013, 2, 10)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2013, 2, 11)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2013, 5, 24)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2013, 6, 1)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2013, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2013, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2013, 11, 2)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2013, 1, 24)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2013, 8, 8)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2013, 8, 9)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2013, 10, 15)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2013, 2, 12)?, "Chinese New Year (in lieu)"),
      (NaiveDate::from_ymd_res(2013, 11, 5)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2014,
    vec![

      (NaiveDate::from_ymd_res(2014, 1, 31)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2014, 2, 1)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2014, 5, 13)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2014, 6, 7)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2014, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2014, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2014, 10, 22)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2014, 1, 14)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2014, 7, 28)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2014, 7, 29)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2014, 10, 5)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2014, 9, 1)?, "National Day (in lieu)"),
      (NaiveDate::from_ymd_res(2014, 10, 6)?, "Hari Raya Haji (in lieu)"),
      (NaiveDate::from_ymd_res(2014, 10, 25)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2015,
    vec![

      (NaiveDate::from_ymd_res(2015, 2, 19)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2015, 2, 20)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2015, 5, 3)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2015, 6, 6)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2015, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2015, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2015, 11, 10)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2015, 1, 3)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2015, 12, 24)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2015, 7, 17)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2015, 7, 18)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2015, 9, 24)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2015, 5, 4)?, "Vesak Day (in lieu)"),
      (NaiveDate::from_ymd_res(2015, 10, 14)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2016,
    vec![

      (NaiveDate::from_ymd_res(2016, 2, 8)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2016, 2, 9)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2016, 5, 21)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2016, 6, 4)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2016, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2016, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2016, 10, 29)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2016, 12, 12)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2016, 7, 6)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2016, 7, 7)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2016, 9, 12)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2016, 5, 2)?, "Labour Day (in lieu)"),
      (NaiveDate::from_ymd_res(2016, 12, 26)?, "Christmas Day (in lieu)"),
      (NaiveDate::from_ymd_res(2016, 10, 2)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2017,
    vec![

      (NaiveDate::from_ymd_res(2017, 1, 28)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2017, 1, 29)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2017, 5, 10)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2017, 6, 3)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2017, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2017, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2017, 10, 18)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2017, 12, 1)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2017, 6, 25)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2017, 6, 26)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2017, 9, 1)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2017, 1, 30)?, "Chinese New Year Holiday (in lieu)"),
      (NaiveDate::from_ymd_res(2017, 6, 27)?, "Hari Raya Puasa (in lieu)"),
      (NaiveDate::from_ymd_res(2017, 9, 22)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2018,
    vec![

      (NaiveDate::from_ymd_res(2018, 2, 16)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2018, 2, 17)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2018, 5, 29)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2018, 9, 9)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2018, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2018, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2018, 11, 6)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2018, 11, 20)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2018, 6, 15)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2018, 6, 16)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2018, 8, 22)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2018, 9, 10)?, "Birthday of SPB Yang di-Pertuan Agong (in lieu)"),
      (NaiveDate::from_ymd_res(2018, 9, 17)?, "Malaysia Day (in lieu)"),
      (NaiveDate::from_ymd_res(2018, 9, 11)?, "Awal Muharram (Hijri New Year)"),
      (NaiveDate::from_ymd_res(2018, 5, 9)?, "Malaysia General Election Holiday"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2019,
    vec![

      (NaiveDate::from_ymd_res(2019, 2, 5)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2019, 2, 6)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2019, 5, 19)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2019, 6, 3)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2019, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2019, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2019, 10, 27)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2019, 11, 9)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2019, 6, 5)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2019, 6, 6)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2019, 8, 11)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2019, 5, 20)?, "Vesak Day (in lieu)"),
      (NaiveDate::from_ymd_res(2019, 8, 12)?, "Hari Raya Haji (in lieu)"),
      (NaiveDate::from_ymd_res(2019, 10, 28)?, "Deepavali (in lieu)"),
      (NaiveDate::from_ymd_res(2019, 9, 1)?, "Awal Muharram (Hijri New Year)"),
      (NaiveDate::from_ymd_res(2019, 7, 30)?, "Installation of New King"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2020,
    vec![

      (NaiveDate::from_ymd_res(2020, 1, 25)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2020, 1, 26)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2020, 5, 7)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2020, 6, 8)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2020, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2020, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2020, 11, 14)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2020, 10, 29)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2020, 5, 24)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2020, 5, 25)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2020, 7, 31)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2020, 1, 27)?, "Chinese New Year Holiday (in lieu)"),
      (NaiveDate::from_ymd_res(2020, 5, 26)?, "Hari Raya Puasa (in lieu)"),
      (NaiveDate::from_ymd_res(2020, 8, 20)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2021,
    vec![

      (NaiveDate::from_ymd_res(2021, 2, 12)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2021, 2, 13)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2021, 5, 26)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2021, 6, 7)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2021, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2021, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2021, 11, 4)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2021, 10, 19)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2021, 5, 13)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2021, 5, 14)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2021, 7, 20)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2021, 8, 10)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2022,
    vec![

      (NaiveDate::from_ymd_res(2022, 2, 1)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2022, 2, 2)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2022, 5, 15)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2022, 6, 6)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2022, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2022, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2022, 10, 24)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2022, 10, 10)?, "Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2022, 5, 2)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2022, 5, 3)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2022, 7, 10)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2022, 5, 4)?, "Labour Day (in lieu)"),
      (NaiveDate::from_ymd_res(2022, 5, 16)?, "Vesak Day (in lieu)"),
      (NaiveDate::from_ymd_res(2022, 7, 11)?, "Hari Raya Haji (in lieu)"),
      (NaiveDate::from_ymd_res(2022, 12, 26)?, "Christmas Day (in lieu)"),
      (NaiveDate::from_ymd_res(2022, 7, 30)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2023,
    vec![

      (NaiveDate::from_ymd_res(2023, 1, 22)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2023, 1, 23)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2023, 5, 4)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2023, 6, 5)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2023, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2023, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2023, 11, 12)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2023, 9, 27)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2023, 4, 22)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2023, 4, 23)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2023, 6, 29)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2023, 1, 24)?, "Chinese New Year (in lieu)"),
      (NaiveDate::from_ymd_res(2023, 4, 24)?, "Second day of Hari Raya Puasa (in lieu)"),
      (NaiveDate::from_ymd_res(2023, 11, 13)?, "Deepavali (in lieu)"),
      (NaiveDate::from_ymd_res(2023, 7, 19)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2024,
    vec![

      (NaiveDate::from_ymd_res(2024, 2, 10)?, "Chinese New Year"),
      (NaiveDate::from_ymd_res(2024, 2, 11)?, "Chinese New Year Holiday"),
      (NaiveDate::from_ymd_res(2024, 5, 22)?, "Vesak Day"),
      (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2024, 6, 3)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2024, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2024, 9, 16)?, "Malaysia Day; Maulidur Rasul (Birthday of the Prophet Muhammad)"),
      (NaiveDate::from_ymd_res(2024, 10, 31)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2024, 4, 10)?, "Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2024, 4, 11)?, "Second day of Hari Raya Puasa"),
      (NaiveDate::from_ymd_res(2024, 6, 17)?, "Hari Raya Haji"),
      (NaiveDate::from_ymd_res(2024, 2, 12)?, "Chinese New Year Holiday (in lieu)"),
      (NaiveDate::from_ymd_res(2024, 7, 7)?, "Awal Muharram (Hijri New Year)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2025,
    vec![

      (NaiveDate::from_ymd_res(2025, 1, 29)?, "Chinese New Year (estimated)"),
      (NaiveDate::from_ymd_res(2025, 1, 30)?, "Chinese New Year Holiday (estimated)"),
      (NaiveDate::from_ymd_res(2025, 5, 11)?, "Vesak Day (estimated)"),
      (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2025, 6, 2)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2025, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2025, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2025, 11, 18)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2025, 9, 4)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2025, 3, 30)?, "Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2025, 3, 31)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2025, 6, 6)?, "Hari Raya Haji (estimated)"),
      (NaiveDate::from_ymd_res(2025, 4, 1)?, "Hari Raya Puasa (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2025, 5, 12)?, "Vesak Day (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2025, 9, 1)?, "National Day (in lieu)"),
      (NaiveDate::from_ymd_res(2025, 6, 26)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2026,
    vec![

      (NaiveDate::from_ymd_res(2026, 2, 17)?, "Chinese New Year (estimated)"),
      (NaiveDate::from_ymd_res(2026, 2, 18)?, "Chinese New Year Holiday (estimated)"),
      (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day; Vesak Day (estimated)"),
      (NaiveDate::from_ymd_res(2026, 6, 1)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2026, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2026, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2026, 11, 7)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2026, 8, 25)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2026, 3, 20)?, "Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2026, 3, 21)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2026, 5, 27)?, "Hari Raya Haji (estimated)"),
      (NaiveDate::from_ymd_res(2026, 6, 16)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2027,
    vec![

      (NaiveDate::from_ymd_res(2027, 2, 6)?, "Chinese New Year (estimated)"),
      (NaiveDate::from_ymd_res(2027, 2, 7)?, "Chinese New Year Holiday (estimated)"),
      (NaiveDate::from_ymd_res(2027, 5, 20)?, "Vesak Day (estimated)"),
      (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2027, 6, 7)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2027, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2027, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2027, 10, 27)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2027, 8, 14)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2027, 3, 9)?, "Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2027, 3, 10)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2027, 5, 16)?, "Hari Raya Haji (estimated)"),
      (NaiveDate::from_ymd_res(2027, 2, 8)?, "Chinese New Year Holiday (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2027, 5, 17)?, "Hari Raya Haji (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2027, 6, 6)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2028,
    vec![

      (NaiveDate::from_ymd_res(2028, 1, 26)?, "Chinese New Year (estimated)"),
      (NaiveDate::from_ymd_res(2028, 1, 27)?, "Chinese New Year Holiday (estimated)"),
      (NaiveDate::from_ymd_res(2028, 5, 9)?, "Vesak Day (estimated)"),
      (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2028, 6, 5)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2028, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2028, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2028, 11, 14)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2028, 8, 3)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2028, 2, 26)?, "Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2028, 2, 27)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2028, 5, 5)?, "Hari Raya Haji (estimated)"),
      (NaiveDate::from_ymd_res(2028, 2, 28)?, "Second day of Hari Raya Puasa (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2028, 5, 25)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2029,
    vec![

      (NaiveDate::from_ymd_res(2029, 2, 13)?, "Chinese New Year (estimated)"),
      (NaiveDate::from_ymd_res(2029, 2, 14)?, "Chinese New Year Holiday (estimated); Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2029, 5, 27)?, "Vesak Day (estimated)"),
      (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2029, 6, 4)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2029, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2029, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2029, 11, 4)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2029, 7, 24)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2029, 2, 15)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2029, 4, 24)?, "Hari Raya Haji (estimated)"),
      (NaiveDate::from_ymd_res(2029, 5, 28)?, "Vesak Day (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2029, 9, 17)?, "Malaysia Day (in lieu)"),
      (NaiveDate::from_ymd_res(2029, 11, 5)?, "Deepavali (in lieu)"),
      (NaiveDate::from_ymd_res(2029, 5, 14)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  build_year(
    years,
    2030,
    vec![

      (NaiveDate::from_ymd_res(2030, 2, 3)?, "Chinese New Year (estimated)"),
      (NaiveDate::from_ymd_res(2030, 2, 4)?, "Chinese New Year Holiday (estimated); Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2030, 5, 16)?, "Vesak Day (estimated)"),
      (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
      (NaiveDate::from_ymd_res(2030, 6, 3)?, "Birthday of SPB Yang di-Pertuan Agong"),
      (NaiveDate::from_ymd_res(2030, 8, 31)?, "National Day"),
      (NaiveDate::from_ymd_res(2030, 9, 16)?, "Malaysia Day"),
      (NaiveDate::from_ymd_res(2030, 10, 25)?, "Deepavali"),
      (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
      (NaiveDate::from_ymd_res(2030, 7, 13)?, "Maulidur Rasul (Birthday of the Prophet Muhammad) (estimated)"),
      (NaiveDate::from_ymd_res(2030, 2, 5)?, "Second day of Hari Raya Puasa (estimated)"),
      (NaiveDate::from_ymd_res(2030, 4, 13)?, "Hari Raya Haji (estimated)"),
      (NaiveDate::from_ymd_res(2030, 2, 6)?, "Chinese New Year (estimated) (in lieu)"),
      (NaiveDate::from_ymd_res(2030, 5, 3)?, "Awal Muharram (Hijri New Year) (estimated)"),
    ],
    &mut map,
    Country::MY,
    "Malaysia",
  );

  Ok(map)
}