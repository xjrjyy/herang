use herang::{init_env, eval};

#[test]
fn test_assign() {
    let mut env = herang::HeEnv::new();
    init_env(&mut env).unwrap();
    let result = eval("a = 1 | 2;", &mut env);
    assert_eq!(result.unwrap().value, vec![1, 2]);
    let result = eval("b = 1 | 1 | 4 | 5 | 1 | 4;", &mut env);
    assert_eq!(result.unwrap().value, vec![1, 1, 4, 5, 1, 4]);
    let result = eval("a = 3 | (b = 1 | 2) | 4 | b;", &mut env);
    assert_eq!(result.unwrap().value, vec![3, 1, 2, 4, 1, 2]);
}

#[test]
fn test_ref_assign() {
    {
        let mut env = herang::HeEnv::new();
        init_env(&mut env).unwrap();
        let result = eval("a = 1 | 2 | 3 | 4 | 5;", &mut env);
        assert!(result.is_ok());
        let result = eval("a[a] = 1 | 2;", &mut env);
        assert_eq!(result.unwrap().value, vec![1, 2, 1, 2, 1]);
    }
    {
        let mut env = herang::HeEnv::new();
        init_env(&mut env).unwrap();
        let result = eval("a = 4 | 2;", &mut env);
        assert_eq!(result.unwrap().value, vec![4, 2]);
        let result = eval("a[0] = 4 | 2;", &mut env);
        assert_eq!(result.unwrap().value, vec![4, 2]);
    }
}

#[test]
fn test_cyber() {
    let mut env = herang::HeEnv::new();
    init_env(&mut env).unwrap();
    let result = eval("a = cyber(5);", &mut env);
    assert!(result.unwrap().value == vec![0, 0, 0, 0, 0]);
}

#[test]
fn test_func() {
    {
        let mut env = herang::HeEnv::new();
        init_env(&mut env).unwrap();

        let result = eval("$cyberfive() { cyber(5); };", &mut env);
        assert!(result.is_ok());

        let result = eval("cyberfive();", &mut env);
        assert!(result.unwrap().value == vec![0, 0, 0, 0, 0]);
    }
    {
        let mut env = herang::HeEnv::new();
        init_env(&mut env).unwrap();

        let result = eval("he = 1 | 2 | 6 | 7;", &mut env);
        assert!(result.is_ok());
        let result = eval("rang = 52 | 57 | 58 | 65;", &mut env);
        assert!(result.is_ok());

        let result = eval("$keyboard(rang) { he = he | 11; rang = 1 | 1 | 4; };", &mut env);
        assert!(result.is_ok());
        let result = eval("rang;", &mut env);
        assert_eq!(result.unwrap().value, vec![52, 57, 58, 65]);

        let result = eval("keyboard(rang);", &mut env);
        assert_eq!(result.unwrap().value, vec![1, 1, 4]);

        let result = eval("he;", &mut env);
        assert_eq!(result.unwrap().value, vec![1, 2, 6, 7, 11]);
        let result = eval("rang;", &mut env);
        assert_eq!(result.unwrap().value, vec![52, 57, 58, 65]);
    }
    {
        let mut env = herang::HeEnv::new();
        init_env(&mut env).unwrap();

        let input = concat!(
            "$A(a) {",
            "    $B(b) {",
            "        b | 3;",
            "    };",
            "    B(a) | 2;",
            "};",
        );
        let result = eval(input, &mut env);
        assert!(result.is_ok());
        let result = eval("A(0 | 1);", &mut env);
        assert_eq!(result.unwrap().value, vec![0, 1, 3, 2]);
    }
}

#[test]
fn test_power_con() {
    let mut env = herang::HeEnv::new();
    init_env(&mut env).unwrap();

    let result = eval("forceCon = cyber(68);", &mut env);
    assert!(result.is_ok());

    let result = eval("$powerCon(whichKey, Force) { forceCon[whichKey] = Force; };", &mut env);
    assert!(result.is_ok());
    let result = eval("powerCon(1 | 2 | 6 | 7 | 11 | 52 | 57 | 58 | 65, 10);", &mut env);
    assert!(result.is_ok());

    let result = eval("forceCon;", &mut env);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value.len(), 68);
    assert!(result.value.iter().filter(|&v| v == &10).count() == 9);
}

#[test]
fn test_if_cond() {
    let mut env = herang::HeEnv::new();
    init_env(&mut env).unwrap();

    let input = concat!(
        "null = cyber(0);",
        "a = 0;",
        "b = 1 | 1 | 0 | 1;",
        "c = 1 | 2 | 3;",
    );

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(null) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![0]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(a) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![0]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(b) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![0]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(c) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![1]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(null < a) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![1]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(a < b) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![1]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(a < c) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![1]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result = 0; ?(b < c) { result = 1; };", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![1]);
}

#[test]
fn test_for_in() {
    let mut env = herang::HeEnv::new();
    init_env(&mut env).unwrap();

    let input = concat!(
        "result = 0;",
        "@(i: 1 | 3 | 4) {",
        "    result = result | i;",
        "};",
    );

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![0, 1, 3, 4]);
}

#[test]
fn test_var_def() {
    let mut env = herang::HeEnv::new();
    init_env(&mut env).unwrap();

    let input = concat!(
        "result = 1;",
    );

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("$A() { def result; result = 2; }; A();", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![1]);

    let result = eval(input, &mut env);
    assert!(result.is_ok());
    let result = eval("$B() { result = 3; }; B();", &mut env);
    assert!(result.is_ok());
    let result = eval("result;", &mut env);
    assert_eq!(result.unwrap().value, vec![3]);
}
