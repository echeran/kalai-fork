use crate::kalai::kalai;
use crate::kalai::kalai::*;
pub fn diff_associative_key(a: kalai::BValue, b: kalai::BValue, k: kalai::BValue) -> kalai::BValue {
    let va: kalai::BValue = kalai::get(a, k);
    let vb: kalai::BValue = kalai::get(b, k);
    let vec_18716: kalai::BValue = diff(va, vb);
    let aa: kalai::BValue = {
        let get1 = get(vec_18716, kalai::BValue::from(0i64));
        if get1.is_some() {
            get1
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let bb: kalai::BValue = {
        let get2 = get(vec_18716, kalai::BValue::from(1i64));
        if get2.is_some() {
            get2
        } else {
            kalai::BValue::from(kalai::NIL)
        }
    };
    let ab: kalai::BValue = {
        let get3 = get(vec_18716, kalai::BValue::from(2i64));
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
    return rpds::Vector::new()
        .push_back(
            if {
                let and_5579_auto: bool = in_a;
                if and_5579_auto {
                    g
                } else {
                    and_5579_auto
                }
            } {
                rpds::HashTrieMap::new().insert(k.clone(), aa.clone())
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
                rpds::HashTrieMap::new().insert(k.clone(), bb.clone())
            } else {
                kalai::BValue::from(kalai::NIL)
            }
            .clone(),
        )
        .push_back(
            if same {
                rpds::HashTrieMap::new().insert(k.clone(), ab.clone())
            } else {
                kalai::BValue::from(kalai::NIL)
            }
            .clone(),
        );
}
pub fn merge2(m1: kalai::BValue, m2: kalai::BValue) -> kalai::BValue {
    return reduce(conj, m1, m2);
}
pub fn diff_associative(a: kalai::BValue, b: kalai::BValue, ks: kalai::BValue) -> kalai::BValue {
    return reduce(
        |diff1, diff2| {
            return std::iter::zip(seq(diff1), seq(diff2))
                .map(|t| |a, b| { merge2(a, b) }(t.0, t.1));
        },
        kalai::BValue::new()
            .push_back(kalai::BValue::from(kalai::NIL).clone())
            .push_back(kalai::BValue::from(kalai::NIL).clone())
            .push_back(kalai::BValue::from(kalai::NIL).clone()),
        seq(ks).clone().into_iter().map(|k| {
            return diff_associative_key(a, b, k);
        }),
    );
}
pub fn union(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if ((s1.len() as i32) < (s2.len() as i32)) {
        return reduce(conj, s2, s1);
    } else {
        return reduce(conj, s1, s2);
    }
}
pub fn difference(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if ((s1.len() as i32) < (s2.len() as i32)) {
        return reduce(
            |result, item| {
                if contains(s2, item) {
                    return result.remove(item);
                } else {
                    return result;
                }
            },
            s1,
            s1,
        );
    } else {
        return reduce(disj, s1, s2);
    }
}
pub fn intersection(s1: kalai::BValue, s2: kalai::BValue) -> kalai::BValue {
    if ((s2.len() as i32) < (s1.len() as i32)) {
        return intersection(s2, s1);
    } else {
        return reduce(
            |result, item| {
                if contains(s2, item) {
                    return result;
                } else {
                    return result.remove(item);
                }
            },
            s1,
            s1,
        );
    }
}
pub fn atom_diff(a: kalai::BValue, b: kalai::BValue) -> kalai::BValue {
    if (a == b) {
        return kalai::BValue::new()
            .push_back(kalai::BValue::from(kalai::NIL).clone())
            .push_back(kalai::BValue::from(kalai::NIL).clone())
            .push_back(a.clone());
    } else {
        return kalai::BValue::new()
            .push_back(a.clone())
            .push_back(b.clone())
            .push_back(kalai::BValue::from(kalai::NIL).clone());
    }
}
pub fn equality_partition(x: kalai::BValue) -> kalai::BValue {
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
    return kalai::BValue::new()
        .push_back(not_empty(difference(a, b)).clone())
        .push_back(not_empty(difference(b, a)).clone())
        .push_back(not_empty(intersection(a, b)).clone());
}
pub fn vectorize(m: kalai::BValue) -> kalai::BValue {
    if seq(m) {
        return reduce(
            |result, p_18759| {
                let vec_18761 = p_18759;
                let k: kalai::BValue = {
                    let get4 = get(vec_18761, kalai::BValue::from(0i64));
                    if get4.is_some() {
                        get4
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                let v: kalai::BValue = {
                    let get5 = get(vec_18761, kalai::BValue::from(1i64));
                    if get5.is_some() {
                        get5
                    } else {
                        kalai::BValue::from(kalai::NIL)
                    }
                };
                return assoc(result, k, v);
            },
            vec(repeat(
                reduce(max, keys(m)),
                kalai::BValue::from(kalai::NIL),
            )),
            m,
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
        range(max((a.len() as i32), (b.len() as i32))),
    )
    .clone()
    .into_iter()
    .map(|a| vectorize(a)));
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
