use crate::kalai::kalai;
use crate::kalai::kalai::*;
pub fn diff_associative_key(a: kalai::BValue, b: kalai::BValue, k: kalai::BValue) -> kalai::BValue {
    let va: kalai::BValue = kalai::get(a.clone(), k.clone());
    let vb: kalai::BValue = kalai::get(b.clone(), k.clone());
    let vec_18746: kalai::BValue = diff(va.clone(), vb.clone());
    let aa: kalai::BValue = {
        let get1 = get(vec_18746.clone(), kalai::BValue::from(0i64));
        if get1.clone().is_some() {
            get1.clone()
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let bb: kalai::BValue = {
        let get2 = get(vec_18746.clone(), kalai::BValue::from(1i64));
        if get2.clone().is_some() {
            get2.clone()
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let ab: kalai::BValue = {
        let get3 = get(vec_18746.clone(), kalai::BValue::from(2i64));
        if get3.clone().is_some() {
            get3.clone()
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let in_a: bool = kalai::contains(a.clone(), k.clone());
    let in_b: bool = kalai::contains(b.clone(), k.clone());
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
        let and_5579_auto_1: bool = in_a.clone();
        if and_5579_auto_1.clone() {
            let and_5579_auto_2: bool = in_b.clone();
            if and_5579_auto_2.clone() {
                c.clone()
            } else {
                and_5579_auto_2.clone()
            }
        } else {
            and_5579_auto_1.clone()
        }
    };
    let e: bool = !aa.clone().is_type("Nil");
    let f: bool = !bb.clone().is_type("Nil");
    let g: bool = {
        let or_5581_auto_1: bool = e.clone();
        if or_5581_auto_1.clone() {
            or_5581_auto_1.clone()
        } else {
            !same.clone()
        }
    };
    let h: bool = {
        let or_5581_auto_2: bool = f.clone();
        if or_5581_auto_2.clone() {
            or_5581_auto_2.clone()
        } else {
            !same.clone()
        }
    };
    return kalai::BValue::from(
        rpds::Vector::new()
            .push_back(
                if {
                    let and_5579_auto_3: bool = in_a.clone();
                    if and_5579_auto_3.clone() {
                        g.clone()
                    } else {
                        and_5579_auto_3.clone()
                    }
                } {
                    kalai::BValue::from(kalai::BValue::from(
                        rpds::HashTrieMap::new().insert(k.clone().clone(), aa.clone().clone()),
                    ))
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            )
            .push_back(
                if {
                    let and_5579_auto_4: bool = in_b.clone();
                    if and_5579_auto_4.clone() {
                        h.clone()
                    } else {
                        and_5579_auto_4.clone()
                    }
                } {
                    kalai::BValue::from(kalai::BValue::from(
                        rpds::HashTrieMap::new().insert(k.clone().clone(), bb.clone().clone()),
                    ))
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            )
            .push_back(
                if same.clone() {
                    kalai::BValue::from(kalai::BValue::from(
                        rpds::HashTrieMap::new().insert(k.clone().clone(), ab.clone().clone()),
                    ))
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone(),
            ),
    );
}
pub fn merge2(m1: kalai::BValue, m2: kalai::BValue) -> kalai::BValue {
    let m2_seq: kalai::BValue = seq(m2.clone());
    return kalai::reduce(
        |o, o2| {
            return kalai::conj(o.clone(), o2.clone());
        },
        m1.clone(),
        m2_seq.clone(),
    );
}
pub fn merge_diffs(diff1: kalai::BValue, diff2: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(kalai::vec(kalai::map2(
        merge2.clone(),
        seq(diff1.clone()),
        seq(diff2.clone()),
    )));
}
pub fn diff_associative(a: kalai::BValue, b: kalai::BValue, ks: kalai::BValue) -> kalai::BValue {
    return kalai::reduce(
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
        return kalai::reduce(conj.clone(), s2.clone(), seq(s1.clone()));
    } else {
        return kalai::reduce(conj.clone(), s1.clone(), seq(s2.clone()));
    }
}
pub fn difference(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s1.clone()) < kalai::count(s2.clone())) {
        return kalai::reduce(
            |result, item| {
                if kalai::contains(s2.clone(), item.clone()) {
                    return kalai::disj(result.clone(), item.clone());
                } else {
                    return result.clone();
                }
            },
            s1.clone(),
            seq(s1.clone()),
        );
    } else {
        return kalai::reduce(disj.clone(), s1.clone(), seq(s2.clone()));
    }
}
pub fn intersection(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if (kalai::count(s2.clone()) < kalai::count(s1.clone())) {
        return intersection(s2.clone(), s1.clone());
    } else {
        return kalai::reduce(
            |result, item| {
                if kalai::contains(s2.clone(), item.clone()) {
                    return result.clone();
                } else {
                    return kalai::disj(result.clone(), item.clone());
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
    let ab_keys: kalai::BValue = union(kalai::keys(a.clone()), kalai::keys(b.clone()));
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
    if kalai::not_empty(m.clone()) {
        return kalai::reduce(
            |result, p_18791| {
                let vec_18793 = p_18791.clone();
                let k: kalai::BValue = {
                    let get4 = get(vec_18793.clone(), kalai::BValue::from(0i64));
                    if get4.clone().is_some() {
                        get4.clone()
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                let v: kalai::BValue = {
                    let get5 = get(vec_18793.clone(), kalai::BValue::from(1i64));
                    if get5.clone().is_some() {
                        get5.clone()
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                return kalai::assoc(result.clone(), k.clone(), v.clone());
            },
            kalai::BValue::from(kalai::vec(repeat(
                i64::from(kalai::reduce(
                    |a, b| {
                        let a_int: i64 = i64::from(a.clone());
                        let b_int: i64 = i64::from(b.clone());
                        return kalai::BValue::from(max(a_int.clone(), b_int.clone()));
                    },
                    kalai::first(seq(kalai::keys(m.clone()))),
                    seq(kalai::keys(m.clone())),
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
    return kalai::BValue::from(kalai::vec(kalai::map(
        |a| vectorize(a.clone()),
        seq(diff_associative(
            if (a.clone().is_type("Vector") || a.clone().is_type("Vec")) {
                a.clone()
            } else {
                kalai::BValue::from(kalai::vec(seq(a.clone())))
            },
            if (b.clone().is_type("Vector") || b.clone().is_type("Vec")) {
                b.clone()
            } else {
                kalai::BValue::from(kalai::vec(seq(b.clone())))
            },
            kalai::BValue::from(kalai::vec(range(max(
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
