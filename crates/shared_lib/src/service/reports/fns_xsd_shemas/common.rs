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



