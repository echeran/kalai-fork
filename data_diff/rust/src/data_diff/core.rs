use crate::kalai::kalai;
use crate::kalai::kalai::*;
pub fn diff_associative_key(a: kalai::BValue, b: kalai::BValue, k: kalai::BValue) -> kalai::BValue {
    let va: kalai::BValue = kalai::get(a, k);
    let vb: kalai::BValue = kalai::get(b, k);
    let vec_18699: kalai::BValue = diff(va, vb);
    let aa: kalai::BValue = {
        let get1 = get(vec_18699, kalai::BValue::from(0i64));
        if get1.is_some() {
            get1
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let bb: kalai::BValue = {
        let get2 = get(vec_18699, kalai::BValue::from(1i64));
        if get2.is_some() {
            get2
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let ab: kalai::BValue = {
        let get3 = get(vec_18699, kalai::BValue::from(2i64));
        if get3.is_some() {
            get3
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let in_a: bool = contains(a, k);
    let in_b: bool = contains(b, k);
    let d: bool = !ab.is_type("Nil");
    let c: bool = {
        let or_5581_auto: bool = d;
        if or_5581_auto {
            or_5581_auto
        } else {
            let and_5579_auto: bool = va.is_type("Nil");
            if and_5579_auto {
                vb.is_type("Nil")
            } else {
                and_5579_auto
            }
        }
    };
    let same: bool = {
        let and_5579_auto: bool = in_a;
        if and_5579_auto {
            let and_5579_auto: bool = in_b;
            if and_5579_auto {
                c
            } else {
                and_5579_auto
            }
        } else {
            and_5579_auto
        }
    };
    let e: bool = !aa.is_type("Nil");
    let f: bool = !bb.is_type("Nil");
    let g: bool = {
        let or_5581_auto: bool = e;
        if or_5581_auto {
            or_5581_auto
        } else {
            !same
        }
    };
    let h: bool = {
        let or_5581_auto: bool = f;
        if or_5581_auto {
            or_5581_auto
        } else {
            !same
        }
    };
    return kalai::BValue::from(
        rpds::Vector::new()
            .push_back(
                if {
                    let and_5579_auto: bool = in_a;
                    if and_5579_auto {
                        g
                    } else {
                        and_5579_auto
                    }
                } {
                    kalai::BValue::from(rpds::HashTrieMap::new().insert(k.clone(), aa.clone()))
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            )
            .push_back(
                if {
                    let and_5579_auto: bool = in_b;
                    if and_5579_auto {
                        h
                    } else {
                        and_5579_auto
                    }
                } {
                    kalai::BValue::from(rpds::HashTrieMap::new().insert(k.clone(), bb.clone()))
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            )
            .push_back(
                if same {
                    kalai::BValue::from(rpds::HashTrieMap::new().insert(k.clone(), ab.clone()))
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            ),
    );
}
pub fn merge2(m1: kalai::BValue, m2: kalai::BValue) -> kalai::BValue {
    return reduce(conj, m1, seq(m2));
}
pub fn diff_associative(a: kalai::BValue, b: kalai::BValue, ks: kalai::BValue) -> kalai::BValue {
    return reduce(
        |diff1, diff2| {
            return kalai::BValue::from(vec(
                std::iter::zip(seq(diff1), seq(diff2)).map(|t| { |a, b| merge2(a, b) }(t.0, t.1))
            ));
        },
        kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone()),
        ),
        seq(ks).map(|k| {
            return diff_associative_key(a, b, k);
        }),
    );
}
pub fn union(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s1) < kalai::count(s2)) {
        return reduce(conj, s2, seq(s1));
    } else {
        return reduce(conj, s1, seq(s2));
    }
}
pub fn difference(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s1) < kalai::count(s2)) {
        return reduce(
            |result, item| {
                if contains(s2, item) {
                    return disj(result, item);
                } else {
                    return result;
                }
            },
            s1,
            seq(s1),
        );
    } else {
        return reduce(disj, s1, seq(s2));
    }
}
pub fn intersection(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s2) < kalai::count(s1)) {
        return intersection(s2, s1);
    } else {
        return reduce(
            |result, item| {
                if contains(s2, item) {
                    return result;
                } else {
                    return disj(result, item);
                }
            },
            s1,
            seq(s1),
        );
    }
}
pub fn atom_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    if (a == b) {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(a.clone()),
        );
    } else {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(a.clone())
                .push_back(b.clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone()),
        );
    }
}
pub fn equality_partition(x: kalai::BValue) -> String {
    if x.is_type("Set") {
        return String::from(":set");
    } else {
        if (x.is_type("Map") || x.is_type("PMap")) {
            return String::from(":map");
        } else {
            if (x.is_type("Vector") || x.is_type("Vec")) {
                return String::from(":sequence");
            } else {
                return String::from(":atom");
            }
        }
    }
}
pub fn map_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    let ab_keys: kalai::BValue = union(keys(a), keys(b));
    return diff_associative(a, b, ab_keys);
}
pub fn set_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(
        rpds::Vector::new()
            .push_back(difference(a, b).clone())
            .push_back(difference(b, a).clone())
            .push_back(intersection(a, b).clone()),
    );
}
pub fn vectorize(m: kalai::BValue) -> kalai::BValue {
    if not_empty(m) {
        return reduce(
            |result, p_18742| {
                let vec_18744 = p_18742;
                let k: kalai::BValue = {
                    let get4 = get(vec_18744, kalai::BValue::from(0i64));
                    if get4.is_some() {
                        get4
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                let v: kalai::BValue = {
                    let get5 = get(vec_18744, kalai::BValue::from(1i64));
                    if get5.is_some() {
                        get5
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                return assoc(result, k, v);
            },
            kalai::BValue::from(vec(repeat(
                (reduce(
                    |a, b| {
                        let a_int: i64 = (a as i64);
                        let b_int: i64 = (b as i64);
                        return kalai::BValue::from(max(a_int, b_int));
                    },
                    seq(keys(m)).next().unwrap().clone(),
                    seq(keys(m)),
                ) as i64),
                kalai::BValue::from(kalai::NIL),
            ))),
            seq(m),
        );
    } else {
        return kalai::BValue::from(kalai::NIL);
    }
}
pub fn sequence_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    return vec(diff_associative(
        if (a.is_type("Vector") || a.is_type("Vec")) {
            a
        } else {
            vec(a)
        },
        if (b.is_type("Vector") || b.is_type("Vec")) {
            b
        } else {
            vec(b)
        },
        range(max(kalai::count(a), kalai::count(b))),
    )
    .map({ |a| vectorize(a) }));
}
pub fn diff_similar(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    let partition_a: kalai::BValue = equality_partition(a);
    let partition_b: kalai::BValue = equality_partition(b);
    if (partition_a == partition_b) {
        if (partition_a == String::from(":set")) {
            return set_diff(a, b);
        } else {
            if (partition_a == String::from(":map")) {
                return map_diff(a, b);
            } else {
                if (partition_a == String::from(":sequence")) {
                    return sequence_diff(a, b);
                } else {
                    if (partition_a == String::from(":atom")) {
                        return atom_diff(a, b);
                    } else {
                        return kalai::BValue::from(kalai::NIL);
                    }
                }
            }
        }
    } else {
        return atom_diff(a, b);
    }
}
pub fn diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    if (a == b) {
        return kalai::BValue::new()
            .push_back(kalai::BValue::from(kalai::NIL).clone())
            .push_back(kalai::BValue::from(kalai::NIL).clone())
            .push_back(a.clone());
    } else {
        return diff_similar(a, b);
    }
}
