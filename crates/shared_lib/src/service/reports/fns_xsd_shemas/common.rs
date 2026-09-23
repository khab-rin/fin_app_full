use crate::make_xls_enum;
use serde::{Serialize, Deserialize};


make_xls_enum!(FnsDocFormVersion, {
	UsnDeclatation => "5.09",
	UsnNotification => "5.02"
});

make_xls_enum!(FnsKnd, {
	UsnDeclatation => "1152017",
	UsnNotification => "1110355"
});


make_xls_enum!(FnsSignerType, {
    TAXPAYER => "1",
    DELEGATE => "2",
});


make_xls_enum!(FnsKbk, {
    UsnSix => "18210501011011000110",
	UsnFifteen => "18210501021011000110"
});

make_xls_enum!(FnsPeriod, {
    FirstQuarter => "21",
    HalfYear     => "31",
    NineMonths   => "33",
    Year         => "34",
});


make_xls_enum!(FnsPeriodNum, {
    QuOne => "01",
    QuTwo => "02",
	QuThree => "03",
    QuFour => "04",
});

