use crate::kalai::kalai;
use crate::kalai::kalai::*;
pub fn diff_associative_key(a: kalai::BValue, b: kalai::BValue, k: kalai::BValue) -> kalai::BValue {
    let va: kalai::BValue = kalai::get(a.clone(), k.clone());
    let vb: kalai::BValue = kalai::get(b.clone(), k.clone());
    let vec_18680: kalai::BValue = diff(va.clone(), vb.clone());
    let aa: kalai::BValue = {
        let get1 = get(vec_18680.clone(), kalai::BValue::from(0i64));
        if get1.clone().is_some() {
            get1.clone()
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let bb: kalai::BValue = {
        let get2 = get(vec_18680.clone(), kalai::BValue::from(1i64));
        if get2.clone().is_some() {
            get2.clone()
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let ab: kalai::BValue = {
        let get3 = get(vec_18680.clone(), kalai::BValue::from(2i64));
        if get3.clone().is_some() {
            get3.clone()
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let in_a: bool = contains(a.clone(), k.clone());
    let in_b: bool = contains(b.clone(), k.clone());
    let d: bool = !ab.clone().is_type("Nil");
    let c: bool = {
        let or_5581_auto: bool = d.clone();
        if or_5581_auto.clone() {
            or_5581_auto.clone()
        } else {
            let and_5579_auto: bool = va.clone().is_type("Nil");
            if and_5579_auto.clone() {
                vb.clone().is_type("Nil")
            } else {
                and_5579_auto.clone()
            }
        }
    };
    let same: bool = {
        let and_5579_auto: bool = in_a.clone();
        if and_5579_auto.clone() {
            let and_5579_auto: bool = in_b.clone();
            if and_5579_auto.clone() {
                c.clone()
            } else {
                and_5579_auto.clone()
            }
        } else {
            and_5579_auto.clone()
        }
    };
    let e: bool = !aa.clone().is_type("Nil");
    let f: bool = !bb.clone().is_type("Nil");
    let g: bool = {
        let or_5581_auto: bool = e.clone();
        if or_5581_auto.clone() {
            or_5581_auto.clone()
        } else {
            !same.clone()
        }
    };
    let h: bool = {
        let or_5581_auto: bool = f.clone();
        if or_5581_auto.clone() {
            or_5581_auto.clone()
        } else {
            !same.clone()
        }
    };
    return kalai::BValue::from(
        rpds::Vector::new()
            .push_back(
                if {
                    let and_5579_auto: bool = in_a.clone();
                    if and_5579_auto.clone() {
                        g.clone()
                    } else {
                        and_5579_auto.clone()
                    }
                } {
                    kalai::BValue::from(
                        rpds::HashTrieMap::new().insert(k.clone().clone(), aa.clone().clone()),
                    )
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            )
            .push_back(
                if {
                    let and_5579_auto: bool = in_b.clone();
                    if and_5579_auto.clone() {
                        h.clone()
                    } else {
                        and_5579_auto.clone()
                    }
                } {
                    kalai::BValue::from(
                        rpds::HashTrieMap::new().insert(k.clone().clone(), bb.clone().clone()),
                    )
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            )
            .push_back(
                if same.clone() {
                    kalai::BValue::from(
                        rpds::HashTrieMap::new().insert(k.clone().clone(), ab.clone().clone()),
                    )
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            ),
    );
}
pub fn merge2(m1: kalai::BValue, m2: kalai::BValue) -> kalai::BValue {
    return reduce(conj.clone(), m1.clone(), seq(m2.clone()));
}
pub fn merge_diffs(diff1: kalai::BValue, diff2: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(vec(kalai::map2(
        merge2.clone(),
        seq(diff1.clone()),
        seq(diff2.clone()),
    )));
}
pub fn diff_associative(a: kalai::BValue, b: kalai::BValue, ks: kalai::BValue) -> kalai::BValue {
    return reduce(
        merge_diffs.clone(),
        kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone()),
        ),
        kalai::map(
            |k| {
                return diff_associative_key(a.clone(), b.clone(), k.clone());
            },
            seq(ks.clone()),
        ),
    );
}
pub fn union(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s1.clone()) < kalai::count(s2.clone())) {
        return reduce(conj.clone(), s2.clone(), seq(s1.clone()));
    } else {
        return reduce(conj.clone(), s1.clone(), seq(s2.clone()));
    }
}
pub fn difference(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s1.clone()) < kalai::count(s2.clone())) {
        return reduce(
            |result, item| {
                if contains(s2.clone(), item.clone()) {
                    return disj(result.clone(), item.clone());
                } else {
                    return result.clone();
                }
            },
            s1.clone(),
            seq(s1.clone()),
        );
    } else {
        return reduce(disj.clone(), s1.clone(), seq(s2.clone()));
    }
}
pub fn intersection(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s2.clone()) < kalai::count(s1.clone())) {
        return intersection(s2.clone(), s1.clone());
    } else {
        return reduce(
            |result, item| {
                if contains(s2.clone(), item.clone()) {
                    return result.clone();
                } else {
                    return disj(result.clone(), item.clone());
                }
            },
            s1.clone(),
            seq(s1.clone()),
        );
    }
}
pub fn atom_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    if (a.clone() == b.clone()) {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(a.clone().clone()),
        );
    } else {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(a.clone().clone())
                .push_back(b.clone().clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone()),
        );
    }
}
pub fn equality_partition(x: kalai::BValue) -> String {
    if x.clone().is_type("Set") {
        return String::from(":set");
    } else {
        if (x.clone().is_type("Map") || x.clone().is_type("PMap")) {
            return String::from(":map");
        } else {
            if (x.clone().is_type("Vector") || x.clone().is_type("Vec")) {
                return String::from(":sequence");
            } else {
                return String::from(":atom");
            }
        }
    }
}
pub fn map_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    let ab_keys: kalai::BValue = union(keys(a.clone()), keys(b.clone()));
    return diff_associative(a.clone(), b.clone(), ab_keys.clone());
}
pub fn set_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(
        rpds::Vector::new()
            .push_back(difference(a.clone(), b.clone()).clone())
            .push_back(difference(b.clone(), a.clone()).clone())
            .push_back(intersection(a.clone(), b.clone()).clone()),
    );
}
pub fn vectorize(m: kalai::BValue) -> kalai::BValue {
    if not_empty(m.clone()) {
        return reduce(
            |result, p_18722| {
                let vec_18724 = p_18722.clone();
                let k: kalai::BValue = {
                    let get4 = get(vec_18724.clone(), kalai::BValue::from(0i64));
                    if get4.clone().is_some() {
                        get4.clone()
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                let v: kalai::BValue = {
                    let get5 = get(vec_18724.clone(), kalai::BValue::from(1i64));
                    if get5.clone().is_some() {
                        get5.clone()
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                return assoc(result.clone(), k.clone(), v.clone());
            },
            kalai::BValue::from(vec(repeat(
                i64::from(reduce(
                    |a, b| {
                        let a_int: i64 = i64::from(a.clone());
                        let b_int: i64 = i64::from(b.clone());
                        return kalai::BValue::from(max(a_int.clone(), b_int.clone()));
                    },
                    first(seq(keys(m.clone()))),
                    seq(keys(m.clone())),
                )),
                kalai::BValue::from(kalai::NIL),
            ))),
            seq(m.clone()),
        );
    } else {
        return kalai::BValue::from(kalai::NIL);
    }
}
pub fn sequence_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(vec(kalai::map(
        |a| vectorize(a.clone()),
        seq(diff_associative(
            if (a.clone().is_type("Vector") || a.clone().is_type("Vec")) {
                a.clone()
            } else {
                kalai::BValue::from(vec(seq(a.clone())))
            },
            if (b.clone().is_type("Vector") || b.clone().is_type("Vec")) {
                b.clone()
            } else {
                kalai::BValue::from(vec(seq(b.clone())))
            },
            kalai::BValue::from(vec(range(max(
                kalai::count(a.clone()),
                kalai::count(b.clone()),
            )))),
        )),
    )));
}
pub fn diff_similar(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    let partition_a: String = equality_partition(a.clone());
    let partition_b: String = equality_partition(b.clone());
    if (partition_a.clone() == partition_b.clone()) {
        if (partition_a.clone() == String::from(":set")) {
            return set_diff(a.clone(), b.clone());
        } else {
            if (partition_a.clone() == String::from(":map")) {
                return map_diff(a.clone(), b.clone());
            } else {
                if (partition_a.clone() == String::from(":sequence")) {
                    return sequence_diff(a.clone(), b.clone());
                } else {
                    if (partition_a.clone() == String::from(":atom")) {
                        return atom_diff(a.clone(), b.clone());
                    } else {
                        return kalai::BValue::from(kalai::NIL);
                    }
                }
            }
        }
    } else {
        return atom_diff(a.clone(), b.clone());
    }
}
pub fn diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    if (a.clone() == b.clone()) {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone())
                .push_back(a.clone().clone()),
        );
    } else {
        return diff_similar(a.clone(), b.clone());
    }
}
