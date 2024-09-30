fn main() {
    work(8, "955");
    work2(8, "996");
    work3("good", "955");
}

/* ================== 基本工作：要求写指定时长的代码 =================== */
fn program(duration: u32) {
    println!("program duration:{}", duration);
}

fn work(time_base: u32, company_type: &str) {
    if company_type == "955" {
        program(time_base * 1);
    } else if company_type == "996" {
        program(time_base * 2);
    } else {
        println!("other company");
    }
}

/* ================== 新需求1：工作内容修改为写ppt =================== */
// 通过函数变量来修改工作内容
fn write_ppt(duration: u32) {
    println!("write_ppt duration:{}", duration);
}
fn work2(time_base: u32, company_type: &str) {
    // 函数作为参数传递，可以动态修改工作内容
    // let action = program;
    let action = write_ppt;
    if company_type == "955" {
        action(time_base * 1);
    } else if company_type == "996" {
        action(time_base * 2);
    } else {
        println!("other company");
    }
}

/* ================== 新需求2：工作内容修改为销售，由时长调整为质量等级 =================== */
// 评价由时长改为质量评价，函数体内容需要修改
fn work3(level: &str, company_type: &str) {
    let action = || {
        println!("company_type:{}, sell product, achieve level:{}", company_type, level);
    };

    if company_type == "955" {
        action();
    } else if company_type == "996" {
        action();
    } else {
        println!("other company");
    }
}