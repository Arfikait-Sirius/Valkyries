static mut GIRL_NAME: String = String::new() ;
static mut SKILL_NAME: String = String::new() ;

//------------------------
// :[ NAME ]:
//     fn_set_GIRL_NAME
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_set_girl_name( name: String ) -> () {
    unsafe {
        GIRL_NAME = name ;
        println!( "{}{}", sfn_get_girls_name_label(), GIRL_NAME ) ;
    }

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
    unsafe {
        SKILL_NAME = target ;
        println!( "{}{}()", sfn_get_target_label(), SKILL_NAME ) ;
    }

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
        unsafe {
            println!( "{}NG - Please check \"{}\".", sfn_get_judgement_label(), SKILL_NAME ) ;
        }
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
