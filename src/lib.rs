#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbg_unimplemented {
    ($($a:expr),*) => {
        unimplemented!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbg_unimplemented {
    () => { {} };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbg_unreachable {
    ($($a:expr),*) => {
        unimplemented!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbg_unreachable {
    () => { {} };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbg_todo {
    ($($a:expr),*) => {
        todo!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbg_todo {
    () => { {} };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbg_panic {
    ($($a:expr),*) => {
        panic!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbg_panic {
    () => { {} };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbg_compile_error {
    ($($a:expr),*) => {
        compile_error!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbg_compile_error {
    () => { {} };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! if_dbg {
    ($a:block) => { $a };
    ($a:expr) => { $a }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! if_dbg {
    ($a:block) => { {} };
    ($a:expr) => { {} };
}

#[cfg(test)]
#[allow(unreachable_code)]
mod tests {
    #[cfg(debug_assertions)]
    #[test]
    fn if_dbg_expr() {
        if_dbg!(return);

        unreachable!();
    }

    #[cfg(debug_assertions)]
    #[test]
    fn if_dbg_block() {
        if_dbg!({
            return;
        });

        unreachable!();
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn if_dbg_expr() {
        if_dbg!(unreahable!());
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn if_dbg_block() {
        if_dbg!({
            unreachable!()
        });
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn todo() {
        dbg_todo!();
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn unimplemented() {
        dbg_unimplemented!()
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn unreachable() {
        dbg_unreachable!();
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn compile_error() {
        dbg_compile_error!();
    }
}

