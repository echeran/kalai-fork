use crate::kalai::kalai;
use crate::kalai::kalai::*;
pub fn diff_associative_key(a: kalai::BValue, b: kalai::BValue, k: kalai::BValue) -> kalai::BValue {
    let va: kalai::BValue = kalai::get(a.clone(), k.clone());
    let vb: kalai::BValue = kalai::get(b.clone(), k.clone());
    let vec_18742: kalai::BValue = diff(va.clone(), vb.clone());
    let aa: kalai::BValue = {
        let get1 = get(vec_18742.clone(), kalai::BValue::from(0i64).clone());
        if get1.is_some() {
            get1
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let bb: kalai::BValue = {
        let get2 = get(vec_18742.clone(), kalai::BValue::from(1i64).clone());
        if get2.is_some() {
            get2
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let ab: kalai::BValue = {
        let get3 = get(vec_18742.clone(), kalai::BValue::from(2i64).clone());
        if get3.is_some() {
            get3
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let in_a: bool = contains(a.clone(), k.clone());
    let in_b: bool = contains(b.clone(), k.clone());
    let d: bool = !ab.is_type("Nil".clone());
    let c: bool = {
        let or_5581_auto: bool = d;
        if or_5581_auto {
            or_5581_auto
        } else {
            let and_5579_auto: bool = va.is_type("Nil".clone());
            if and_5579_auto {
                vb.is_type("Nil".clone())
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
    let e: bool = !aa.is_type("Nil".clone());
    let f: bool = !bb.is_type("Nil".clone());
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
                    kalai::BValue::from(
                        rpds::HashTrieMap::new()
                            .insert(k.clone().clone(), aa.clone().clone())
                            .clone(),
                    )
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone()
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
                    kalai::BValue::from(
                        rpds::HashTrieMap::new()
                            .insert(k.clone().clone(), bb.clone().clone())
                            .clone(),
                    )
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone()
                .clone(),
            )
            .push_back(
                if same {
                    kalai::BValue::from(
                        rpds::HashTrieMap::new()
                            .insert(k.clone().clone(), ab.clone().clone())
                            .clone(),
                    )
                } else {
                    kalai::BValue::from(kalai::NIL)
                }
                .clone()
                .clone(),
            )
            .clone(),
    );
}
pub fn merge2(m1: kalai::BValue, m2: kalai::BValue) -> kalai::BValue {
    return reduce(conj.clone(), m1.clone(), seq(m2.clone()));
}
pub fn merge_diffs(diff1: kalai::BValue, diff2: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(
        vec(std::iter::zip(seq(diff1.clone()), seq(diff2.clone()))
            .map(|t| { |a, b| merge2(a.clone(), b.clone()) }(t.0.clone(), t.1.clone()).clone()))
        .clone(),
    );
}
pub fn diff_associative(a: kalai::BValue, b: kalai::BValue, ks: kalai::BValue) -> kalai::BValue {
    return reduce(
        merge_diffs.clone(),
        kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .clone(),
        )
        .clone(),
        seq(ks.clone()).map(|k| {
            {
                return diff_associative_key(a.clone(), b.clone(), k.clone());
            }
            .clone()
        }),
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
                {
                    if contains(s2.clone(), item.clone()) {
                        return disj(result.clone(), item.clone());
                    } else {
                        return result;
                    }
                }
                .clone()
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
                {
                    if contains(s2.clone(), item.clone()) {
                        return result;
                    } else {
                        return disj(result.clone(), item.clone());
                    }
                }
                .clone()
            },
            s1.clone(),
            seq(s1.clone()),
        );
    }
}
pub fn atom_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    if (a == b) {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .push_back(a.clone().clone())
                .clone(),
        );
    } else {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(a.clone().clone())
                .push_back(b.clone().clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .clone(),
        );
    }
}
pub fn equality_partition(x: kalai::BValue) -> String {
    if x.is_type("Set".clone()) {
        return String::from(":set");
    } else {
        if (x.is_type("Map".clone()) || x.is_type("PMap".clone())) {
            return String::from(":map");
        } else {
            if (x.is_type("Vector".clone()) || x.is_type("Vec".clone())) {
                return String::from(":sequence");
            } else {
                return String::from(":atom");
            }
        }
    }
}
pub fn map_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    let ab_keys: kalai::BValue = union(keys(a.clone()).clone(), keys(b.clone()).clone());
    return diff_associative(a.clone(), b.clone(), ab_keys.clone());
}
pub fn set_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(
        rpds::Vector::new()
            .push_back(difference(a.clone(), b.clone()).clone().clone())
            .push_back(difference(b.clone(), a.clone()).clone().clone())
            .push_back(intersection(a.clone(), b.clone()).clone().clone())
            .clone(),
    );
}
pub fn vectorize(m: kalai::BValue) -> kalai::BValue {
    if not_empty(m.clone()) {
        return reduce(
            |result, p_18784| {
                {
                    let vec_18786 = p_18784;
                    let k: kalai::BValue = {
                        let get4 = get(vec_18786.clone(), kalai::BValue::from(0i64).clone());
                        if get4.is_some() {
                            get4
                        } else {
                            kalai::BValue::from(kalai::NIL)
                        }
                    };
                    let v: kalai::BValue = {
                        let get5 = get(vec_18786.clone(), kalai::BValue::from(1i64).clone());
                        if get5.is_some() {
                            get5
                        } else {
                            kalai::BValue::from(kalai::NIL)
                        }
                    };
                    return assoc(result.clone(), k.clone(), v.clone());
                }
                .clone()
            },
            kalai::BValue::from(
                vec(repeat(
                    i64::from(
                        reduce(
                            |a, b| {
                                {
                                    let a_int: i64 = i64::from(a.clone());
                                    let b_int: i64 = i64::from(b.clone());
                                    return kalai::BValue::from(
                                        max(a_int.clone(), b_int.clone()).clone(),
                                    );
                                }
                                .clone()
                            },
                            seq(keys(m.clone()).clone()).next().unwrap().clone().clone(),
                            seq(keys(m.clone()).clone()),
                        )
                        .clone(),
                    )
                    .clone(),
                    kalai::BValue::from(kalai::NIL).clone(),
                ))
                .clone(),
            )
            .clone(),
            seq(m.clone()),
        );
    } else {
        return kalai::BValue::from(kalai::NIL);
    }
}
pub fn sequence_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    return kalai::BValue::from(
        vec(seq(diff_associative(
            if (a.is_type("Vector".clone()) || a.is_type("Vec".clone())) {
                a
            } else {
                kalai::BValue::from(vec(seq(a.clone())).clone())
            }
            .clone(),
            if (b.is_type("Vector".clone()) || b.is_type("Vec".clone())) {
                b
            } else {
                kalai::BValue::from(vec(seq(b.clone())).clone())
            }
            .clone(),
            kalai::BValue::from(
                vec(range(
                    max(
                        kalai::count(a.clone()).clone(),
                        kalai::count(b.clone()).clone(),
                    )
                    .clone(),
                ))
                .clone(),
            )
            .clone(),
        )
        .clone())
        .map({ |a| vectorize(a.clone()) }.clone()))
        .clone(),
    );
}
pub fn diff_similar(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    let partition_a: String = equality_partition(a.clone());
    let partition_b: String = equality_partition(b.clone());
    if (partition_a == partition_b) {
        if (partition_a == String::from(":set")) {
            return set_diff(a.clone(), b.clone());
        } else {
            if (partition_a == String::from(":map")) {
                return map_diff(a.clone(), b.clone());
            } else {
                if (partition_a == String::from(":sequence")) {
                    return sequence_diff(a.clone(), b.clone());
                } else {
                    if (partition_a == String::from(":atom")) {
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
    if (a == b) {
        return kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .push_back(kalai::BValue::from(kalai::NIL).clone().clone())
                .push_back(a.clone().clone())
                .clone(),
        );
    } else {
        return diff_similar(a.clone(), b.clone());
    }
}
