//------------------------
// :[ NAME ]:
//     fn_count
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_count( base: String, target: String ) -> usize {
    let mut result = 0 ;
    let mut tmp = base ;

    let mut is_contain = tmp.contains( &target ) ;
    while is_contain {
        result += 1 ;
        tmp = tmp.replacen( &target, "", 1 ) ;
        is_contain = tmp.contains( &target ) ;
    }

    return result ;
}

//------------------------
// :[ NAME ]:
//     fn_copy
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_copy( base: String ) -> String {
    return base ;
}

//------------------------
// :[ NAME ]:
//     fn_replace
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_replace( base: String, target: String, replacement: String ) -> String {
    return base.replacen( &target, &replacement, 1 ).to_string() ;
}

//------------------------
// :[ NAME ]:
//     fn_split
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_split( base: String, target: String, index: usize ) -> String {
    let result: Vec<&str> = base.split( &target ).collect() ;
    return result[index].to_string() ;
}

//------------------------
// :[ NAME ]:
//     fn_upper_all
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
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
#[allow( dead_code )]
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
#[allow( dead_code )]
pub fn fn_upper_first( base: String ) -> String {
    let base_upper = base.to_uppercase() ;
    let base_lower = base.to_lowercase() ;

    return format!( "{}{}", &base_upper[..1], &base_lower[1..base_lower.len()] ) ;
}

//------------------------
// :[ NAME ]:
//     fn_get_length
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_get_length( base: String ) -> usize {
    return base.len() ;
}
