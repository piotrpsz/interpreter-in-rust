#![allow(dead_code)]

extern crate libc;
use std::fmt;
use std::mem;

const SECONDS_IN_MINUTE: i64 = 60;
const SECONDS_IN_HOUR  : i64 = 60 * SECONDS_IN_MINUTE;
const SECONDS_IN_DAY   : i64 = 24 * SECONDS_IN_HOUR;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Region {
   Utc,
   Local,
}

#[derive(Copy, Clone)]
pub struct DateTime {
   region: Region,
   tstamp: i64,
   year  : u16,
   month : u8,
   day   : u8,
   hour  : u8,
   min   : u8,
   sec   : u8,
   wday  : u8,
   yday  : u16,
}

/// Konwersja unix'owej struktury tm na strukture DateTime.
fn dt_from_tm(region: Region, ts: libc::time_t, tm: &libc::tm) -> DateTime {
   let tstamp = ts;
   let year = (tm.tm_year + 1900) as u16;
   let month = (tm.tm_mon + 1) as u8;
   let day = tm.tm_mday as u8;
   let hour = tm.tm_hour as u8;
   let min = tm.tm_min as u8;
   let sec = tm.tm_sec as u8;
   let wday = tm.tm_wday as u8;
   let yday = tm.tm_yday as u16;

   DateTime { region, tstamp, year,month, day, hour, min, sec, wday, yday }
}

pub fn now(region: Region) -> Option<DateTime> {
   if let Some(ts) = timestamp() {
      let mut tm: libc::tm =  unsafe { mem::zeroed() };

      if region == Region::Local {
         if local(&ts, &mut tm) {
            return Some(dt_from_tm(region, ts, &tm));
         }
      }
      else {
         if utc(&ts, &mut tm) {
            return Some(dt_from_tm(region, ts, &tm));
         }
      }
   }
   None
}

pub fn local_from_tstamp(ts: i64) -> Option<DateTime> {
   let mut tm: libc::tm =  unsafe { mem::zeroed() };
   if local(&ts, &mut tm) {
      return Some(dt_from_tm(Region::Local, ts, &tm));
   }           
   None
}

pub fn utc_from_tstamp(ts: i64) -> Option<DateTime> {
   let mut tm: libc::tm =  unsafe { mem::zeroed() };
   if utc(&ts, &mut tm) {
      return Some(dt_from_tm(Region::Utc, ts, &tm));
   }           
   None
}



#[inline]
fn timestamp() -> Option<libc::time_t> {
   unsafe {
      let ts = libc::time(std::ptr::null_mut());
      if ts != -1 {
         return Some(ts);
      }
      None
   }
}

#[inline]
fn utc(t: &libc::time_t, tm: &mut libc::tm) -> bool {
   unsafe {
      !libc::gmtime_r(t, tm).is_null()
   }
}

#[inline]
fn local(t: &libc::time_t, tm: &mut libc::tm) -> bool {
   unsafe {
      !libc::localtime_r(t, tm).is_null()
   }
}


impl DateTime {

   pub fn timestamp(&self) -> i64 {
      self.tstamp
   }

   /// Zwraca numer dnia w tygodniu (1..7 odpowiada poniedziałek..niedziela)
   pub fn day_of_week(&self) -> isize {
      if self.wday == 0 {
         return 7;
      }
      self.wday as isize
   }

   /// Zwraca numer dnia w roku (1..)
   pub fn day_of_year(&self) -> isize {
      (self.yday as isize) + 1
   }

   pub fn year(&self) -> isize {
      self.year as isize
   }

   pub fn week_of_year(&self) -> isize {
      let n = ((self.yday as isize) + 1) / 7;
      if (n % 7) != 0 {
         return n + 1;
      }
      n
   }

   #[inline]
   pub fn is_utc(&self) -> bool {
      self.region == Region::Utc
   }

   #[inline]
   pub fn is_local(&self) -> bool {
      self.region == Region::Local
   }

   /// Utworzenie obiektu w regionie UTC (jeśli obiekt jest Local).
   /// Jeśli obiekt już jest UTC zwracana jest jego kopia.
   pub fn as_utc(&self) -> DateTime {
      if self.is_local() {
         let ts = self.tstamp;
         let mut tm: libc::tm =  unsafe { mem::zeroed() };
         if utc(&ts, &mut tm) {
            return dt_from_tm(Region::Utc, ts, &tm);
         }
      }
      self.clone()
   }

   /// Utworzenie obiektu w regionie Local (jeśli obiekt jest UTC).
   /// Jeśli obiekt już jest Local zwracana jest jego kopia.
   pub fn as_local(&self) -> DateTime {
      if self.is_utc() {
         let ts = self.tstamp;
         let mut tm: libc::tm =  unsafe { mem::zeroed() };
         if local(&ts, &mut tm) {
            return dt_from_tm(Region::Utc, ts, &tm);
         }         
      }
      self.clone()
   }

   
   pub fn add_seconds(&self, n: i64) -> DateTime {
      let ts = self.tstamp + n;
      let mut tm: libc::tm =  unsafe { mem::zeroed() };
      match self.region {
         Region::Local => {
            local(&ts, &mut tm);
            dt_from_tm(Region::Local, ts, &tm)
         }
         _ => {
            utc(&ts, &mut tm);
            dt_from_tm(Region::Utc, ts, &tm)
         }
      }
   }

   pub fn add_minutes(&self, n: i64) -> DateTime {
      let ts = self.tstamp + (n * SECONDS_IN_MINUTE);
      let mut tm: libc::tm =  unsafe { mem::zeroed() };
      match self.region {
         Region::Local => {
            local(&ts, &mut tm);
            dt_from_tm(Region::Local, ts, &tm)
         }
         _ => {
            utc(&ts, &mut tm);
            dt_from_tm(Region::Utc, ts, &tm)
         }
      }
   }

   pub fn add_days(&self, n: i64) -> DateTime {
      let ts = self.tstamp + (n * SECONDS_IN_DAY);
      let mut tm: libc::tm =  unsafe { mem::zeroed() };
      match self.region {
         Region::Local => {
            local(&ts, &mut tm);
            dt_from_tm(Region::Local, ts, &tm)
         }
         _ => {
            utc(&ts, &mut tm);
            dt_from_tm(Region::Utc, ts, &tm)
         }
      }
   }

   pub fn as_string(&self) -> String {
      format!("{}/{:02}/{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.min, self.sec)
   }
}



impl fmt::Display for DateTime {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?} {}/{:02}/{:02} {:02}:{:02}:{:02} ({})", self.region, self.year, self.month, self.day, self.hour, self.min, self.sec, self.tstamp)
   }
}
