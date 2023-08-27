(ns kalai.pass.rust.ab-cast
  (:require [meander.strategy.epsilon :as s]
            [meander.epsilon :as m]
            [kalai.pass.rust.e-string :as e-string]
            [kalai.util :as u]))

(defn rust-cast-type?
  "When transpiling a :cast annotation, given the \"from\" and \"to\" type of the cast,
  return whether the Rust output should use the `as` identifier.
  If false, then the effect of this pass will be to execute the rules
  to output the `::from()` syntax for helper
  `From` trait implementations."
  [t]
  (and (some? t) (not= :any t)))

(def rewrite
  (s/bottom-up
    (s/rewrite
      ;; This is Rust style for primitive type casting (x as T)
      (m/and ?x
             (m/app meta {:t (m/pred rust-cast-type?)
                          :cast (m/pred rust-cast-type? ?cast)}))
      (r/cast ?x ?cast)

      ;; Support type casting between `BValue`(`:any`) and some specific type.
      ;; This style of casting is done by helper method from kalai.rs
      ;; Example: bool::from(x) which gives a bool from an x of type BValue
      ;; Note: this applies for when the Kalai type is either :any
      (m/and ?x
             (m/app meta {:cast (m/pred some? ?t)}))
      (r/invoke ~(str (e-string/init-rhs-t-str ?t) "::from") ?x)

      ?else
      ?else)))
