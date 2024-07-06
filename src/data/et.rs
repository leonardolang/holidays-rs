
//! Ethiopia
use super::*;

/// Generate holiday map for Ethiopia.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
  let mut map = HashMap::new();

  build_year(
    years,
    2000,
    vec![

      (NaiveDate::from_ymd_res(2000, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2000, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2000, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2000, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2000, 4, 28)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2000, 4, 30)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2000, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2000, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2000, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2000, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2000, 1, 8)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2000, 12, 27)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2000, 3, 16)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2000, 3, 17)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2000, 6, 14)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2000, 6, 15)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2001,
    vec![

      (NaiveDate::from_ymd_res(2001, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2001, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2001, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2001, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2001, 4, 13)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2001, 4, 15)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2001, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2001, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2001, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2001, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2001, 12, 16)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2001, 3, 5)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2001, 3, 6)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2001, 6, 4)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2001, 6, 5)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2002,
    vec![

      (NaiveDate::from_ymd_res(2002, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2002, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2002, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2002, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2002, 5, 3)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2002, 5, 5)?, "የአርበኞች ቀን; ፋሲካ"),
      (NaiveDate::from_ymd_res(2002, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2002, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2002, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2002, 12, 5)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2002, 2, 22)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2002, 2, 23)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2002, 5, 24)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2002, 5, 25)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2003,
    vec![

      (NaiveDate::from_ymd_res(2003, 9, 12)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2003, 9, 28)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2003, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2003, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2003, 4, 25)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2003, 4, 27)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2003, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2003, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2003, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2003, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2003, 11, 25)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2003, 2, 11)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2003, 2, 12)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2003, 5, 13)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2003, 5, 14)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2004,
    vec![

      (NaiveDate::from_ymd_res(2004, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2004, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2004, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2004, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2004, 4, 9)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2004, 4, 11)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2004, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2004, 5, 1)?, "መውሊድ (ግምት); የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2004, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2004, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2004, 11, 14)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2004, 2, 1)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2004, 2, 2)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2004, 5, 2)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2005,
    vec![

      (NaiveDate::from_ymd_res(2005, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2005, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2005, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2005, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2005, 4, 29)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2005, 5, 1)?, "የሰራተኞች ቀን; ፋሲካ"),
      (NaiveDate::from_ymd_res(2005, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2005, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2005, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2005, 11, 3)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2005, 1, 21)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2005, 1, 22)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2005, 4, 21)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2005, 4, 22)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2006,
    vec![

      (NaiveDate::from_ymd_res(2006, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2006, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2006, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2006, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2006, 4, 21)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2006, 4, 23)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2006, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2006, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2006, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2006, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2006, 10, 23)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2006, 1, 10)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2006, 12, 31)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2006, 1, 11)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2006, 4, 10)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2006, 4, 11)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2007,
    vec![

      (NaiveDate::from_ymd_res(2007, 9, 12)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2007, 9, 28)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2007, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2007, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2007, 4, 6)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2007, 4, 8)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2007, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2007, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2007, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2007, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2007, 10, 13)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2007, 12, 20)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2007, 1, 1)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2007, 12, 21)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2007, 3, 31)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2007, 4, 1)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2008,
    vec![

      (NaiveDate::from_ymd_res(2008, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2008, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2008, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2008, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2008, 4, 25)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2008, 4, 27)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2008, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2008, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2008, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2008, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2008, 10, 1)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2008, 12, 8)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2008, 12, 9)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2008, 3, 20)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2008, 3, 21)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2009,
    vec![

      (NaiveDate::from_ymd_res(2009, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2009, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2009, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2009, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2009, 4, 17)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2009, 4, 19)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2009, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2009, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2009, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2009, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2009, 9, 20)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2009, 11, 27)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2009, 11, 28)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2009, 3, 9)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2009, 3, 10)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2010,
    vec![

      (NaiveDate::from_ymd_res(2010, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2010, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2010, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2010, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2010, 4, 2)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2010, 4, 4)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2010, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2010, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2010, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2010, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2010, 9, 10)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2010, 11, 16)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2010, 11, 17)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2010, 2, 26)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2010, 2, 27)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2011,
    vec![

      (NaiveDate::from_ymd_res(2011, 9, 12)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2011, 9, 28)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2011, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2011, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2011, 4, 22)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2011, 4, 24)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2011, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2011, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2011, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2011, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2011, 8, 30)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2011, 11, 6)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2011, 11, 7)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2011, 2, 15)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2011, 2, 16)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2012,
    vec![

      (NaiveDate::from_ymd_res(2012, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2012, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2012, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2012, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2012, 4, 13)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2012, 4, 15)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2012, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2012, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2012, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2012, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2012, 8, 19)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2012, 10, 26)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2012, 10, 27)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2012, 2, 4)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2012, 2, 5)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2013,
    vec![

      (NaiveDate::from_ymd_res(2013, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2013, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2013, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2013, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2013, 5, 3)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2013, 5, 5)?, "የአርበኞች ቀን; ፋሲካ"),
      (NaiveDate::from_ymd_res(2013, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2013, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2013, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2013, 8, 8)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2013, 10, 15)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2013, 10, 16)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2013, 1, 24)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2013, 1, 25)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2014,
    vec![

      (NaiveDate::from_ymd_res(2014, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2014, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2014, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2014, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2014, 4, 18)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2014, 4, 20)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2014, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2014, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2014, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2014, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2014, 7, 28)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2014, 10, 4)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2014, 10, 5)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2014, 1, 13)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2014, 1, 14)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2015,
    vec![

      (NaiveDate::from_ymd_res(2015, 9, 12)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2015, 9, 28)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2015, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2015, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2015, 4, 10)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2015, 4, 12)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2015, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2015, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2015, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2015, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2015, 7, 17)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2015, 9, 23)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2015, 9, 24)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2015, 1, 3)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2015, 12, 23)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2015, 1, 4)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2015, 12, 24)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2016,
    vec![

      (NaiveDate::from_ymd_res(2016, 9, 11)?, "አረፋ (ግምት); አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2016, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2016, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2016, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2016, 4, 29)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2016, 5, 1)?, "የሰራተኞች ቀን; ፋሲካ"),
      (NaiveDate::from_ymd_res(2016, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2016, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2016, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2016, 7, 6)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2016, 9, 12)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2016, 12, 11)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2016, 12, 12)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2017,
    vec![

      (NaiveDate::from_ymd_res(2017, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2017, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2017, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2017, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2017, 4, 14)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2017, 4, 16)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2017, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2017, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2017, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2017, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2017, 6, 25)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2017, 9, 1)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2017, 9, 2)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2017, 11, 30)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2017, 12, 1)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2018,
    vec![

      (NaiveDate::from_ymd_res(2018, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2018, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2018, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2018, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2018, 4, 6)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2018, 4, 8)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2018, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2018, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2018, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2018, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2018, 6, 15)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2018, 8, 21)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2018, 8, 22)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2018, 11, 20)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2018, 11, 21)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2019,
    vec![

      (NaiveDate::from_ymd_res(2019, 9, 12)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2019, 9, 28)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2019, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2019, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2019, 4, 26)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2019, 4, 28)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2019, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2019, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2019, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2019, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2019, 6, 4)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2019, 8, 11)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2019, 8, 12)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2019, 11, 9)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2019, 11, 10)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2020,
    vec![

      (NaiveDate::from_ymd_res(2020, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2020, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2020, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2020, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2020, 4, 17)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2020, 4, 19)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2020, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2020, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2020, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2020, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2020, 5, 24)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2020, 7, 31)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2020, 8, 1)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2020, 10, 29)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2020, 10, 30)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2021,
    vec![

      (NaiveDate::from_ymd_res(2021, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2021, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2021, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2021, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2021, 4, 30)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2021, 5, 2)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2021, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2021, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2021, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2021, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2021, 5, 13)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2021, 7, 20)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2021, 7, 21)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2021, 10, 18)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2021, 10, 19)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2022,
    vec![

      (NaiveDate::from_ymd_res(2022, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2022, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2022, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2022, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2022, 4, 22)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2022, 4, 24)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2022, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2022, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2022, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2022, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2022, 5, 2)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2022, 7, 9)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2022, 7, 10)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2022, 10, 8)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2022, 10, 9)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2023,
    vec![

      (NaiveDate::from_ymd_res(2023, 9, 12)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2023, 9, 28)?, "መስቀል; መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2023, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2023, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2023, 4, 14)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2023, 4, 16)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2023, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2023, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2023, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2023, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2023, 4, 21)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2023, 6, 28)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2023, 6, 29)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2023, 9, 27)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2024,
    vec![

      (NaiveDate::from_ymd_res(2024, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2024, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2024, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2024, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2024, 5, 3)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2024, 5, 5)?, "የአርበኞች ቀን; ፋሲካ"),
      (NaiveDate::from_ymd_res(2024, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2024, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2024, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2024, 4, 10)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2024, 6, 16)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2024, 6, 17)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2024, 9, 15)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2024, 9, 16)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2025,
    vec![

      (NaiveDate::from_ymd_res(2025, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2025, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2025, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2025, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2025, 4, 18)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2025, 4, 20)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2025, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2025, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2025, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2025, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2025, 3, 30)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2025, 6, 6)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2025, 6, 7)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2025, 9, 4)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2025, 9, 5)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2026,
    vec![

      (NaiveDate::from_ymd_res(2026, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2026, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2026, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2026, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2026, 4, 10)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2026, 4, 12)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2026, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2026, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2026, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2026, 5, 28)?, "አረፋ (ግምት); ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2026, 3, 20)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2026, 5, 27)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2026, 8, 25)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2026, 8, 26)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2027,
    vec![

      (NaiveDate::from_ymd_res(2027, 9, 12)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2027, 9, 28)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2027, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2027, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2027, 4, 30)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2027, 5, 2)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2027, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2027, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2027, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2027, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2027, 3, 9)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2027, 5, 16)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2027, 5, 17)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2027, 8, 14)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2027, 8, 15)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2028,
    vec![

      (NaiveDate::from_ymd_res(2028, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2028, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2028, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2028, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2028, 4, 14)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2028, 4, 16)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2028, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2028, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2028, 5, 5)?, "አረፋ (ግምት); የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2028, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2028, 2, 26)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2028, 5, 6)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2028, 8, 3)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2028, 8, 4)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2029,
    vec![

      (NaiveDate::from_ymd_res(2029, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2029, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2029, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2029, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2029, 4, 6)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2029, 4, 8)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2029, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2029, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2029, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2029, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2029, 2, 14)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2029, 4, 24)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2029, 4, 25)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2029, 7, 24)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2029, 7, 25)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  build_year(
    years,
    2030,
    vec![

      (NaiveDate::from_ymd_res(2030, 9, 11)?, "አዲስ ዓመት እንቁጣጣሽ"),
      (NaiveDate::from_ymd_res(2030, 9, 27)?, "መስቀል"),
      (NaiveDate::from_ymd_res(2030, 1, 7)?, "ገና"),
      (NaiveDate::from_ymd_res(2030, 1, 19)?, "ጥምቀት"),
      (NaiveDate::from_ymd_res(2030, 4, 26)?, "ስቅለት"),
      (NaiveDate::from_ymd_res(2030, 4, 28)?, "ፋሲካ"),
      (NaiveDate::from_ymd_res(2030, 3, 2)?, "አድዋ"),
      (NaiveDate::from_ymd_res(2030, 5, 1)?, "የሰራተኞች ቀን"),
      (NaiveDate::from_ymd_res(2030, 5, 5)?, "የአርበኞች ቀን"),
      (NaiveDate::from_ymd_res(2030, 5, 28)?, "ደርግ የወደቀበት ቀን"),
      (NaiveDate::from_ymd_res(2030, 2, 4)?, "ኢድ አልፈጥር (ግምት)"),
      (NaiveDate::from_ymd_res(2030, 4, 13)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2030, 4, 14)?, "አረፋ (ግምት)"),
      (NaiveDate::from_ymd_res(2030, 7, 13)?, "መውሊድ (ግምት)"),
      (NaiveDate::from_ymd_res(2030, 7, 14)?, "መውሊድ (ግምት)"),
    ],
    &mut map,
    Country::ET,
    "Ethiopia",
  );

  Ok(map)
}