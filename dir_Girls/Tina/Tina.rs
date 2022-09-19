//------------------------
// :[ NAME ]:
//     fn_set_GIRL_NAME
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_set_girl_name( name: String ) -> () {
    
    println!( "{}{}", sfn_get_girls_name_label(), name ) ;

    return ;
}

//------------------------
// :[ NAME ]:
//     fn_set_SKILL_NAME
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_set_skill_name( target: String ) -> () {
    
    println!( "{}{}()", sfn_get_target_label(), target ) ;

    return ;
}

//------------------------
// :[ NAME ]:
//     fn_judge
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_judge( judgement: bool ) -> () {

    if judgement {
        println!( "{}OK", sfn_get_judgement_label() ) ;
    }else{
        println!( "{}NG - Please check this source.", sfn_get_judgement_label() ) ;
    }

    return ;
}

fn sfn_get_girls_name_label() -> String {
    return "[GIRLS-NAME]: ".to_string() ;
}
fn sfn_get_target_label() -> String {
    return "[    TARGET]: ".to_string() ;
}
fn sfn_get_judgement_label() -> String {
    return "[     JUDGE]: ".to_string() ;
}
