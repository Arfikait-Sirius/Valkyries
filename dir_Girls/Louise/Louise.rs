//------------------------
// :[ NAME ]:
//     fn_count
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_count( base: String, target: String ) -> usize {
    return base.match_indices( &target ).count() ;
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
//     fn_get_substring
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_get_substring( base: String, format: String ) -> String {
    let index: usize ;
    let head_index: usize ;
    let tail_index: usize ;

    match format.find( "%s" ){
        Some( v ) => index = v,
        None => return "".to_string()
    }
    let head = &format[..index] ;
    let tail = &format[( index + 2 )..] ;

    match base.find( &head ){
        Some( v ) => head_index = v + head.len(),
        None => return "".to_string()
    }
    match base.find( &tail ){
        Some( v ) => tail_index = v,
        None => return "".to_string()
    }

    return ( &base[head_index..tail_index] ).to_string() ;
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

    return format!( "{}{}", &base_upper[..1], &base_lower[1..] ) ;
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
