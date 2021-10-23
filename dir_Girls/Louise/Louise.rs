//------------------------
// :[ NAME ]:
//     fn_upper_all
//
// :[ CATEGORY ]:
//     Skill
//------------------------
pub fn fn_upper_all( base: String ) -> String {
    return base.to_uppercase() ;
}

//------------------------
// :[ NAME ]:
//     fn_lower_all
//
// :[ CATEGORY ]:
//     Skill
//------------------------
pub fn fn_lower_all( base: String ) -> String {
    return base.to_lowercase() ;
}

//------------------------
// :[ NAME ]:
//     fn_upper_first
//
// :[ CATEGORY ]:
//     Skill
//------------------------
pub fn fn_upper_first( base: String ) -> String {
    let base_upper = base.to_uppercase() ;
    let base_lower = base.to_lowercase() ;

    return format!( "{}{}", &base_upper[..1], &base_lower[1..base_lower.len()] ) ;
}
