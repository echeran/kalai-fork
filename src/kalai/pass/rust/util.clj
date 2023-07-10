(ns kalai.pass.rust.util
  (:require [kalai.types :as types]
            [kalai.util :as u]
            [clojure.string :as str]))

(defn clone
  "Preserves the type information while wrapping a value in a clone method"
  [expr]
  (u/preserve-type expr (list 'r/method 'clone expr)))


(defn literal? [x]
  (or (number? x)
      (string? x)
      (keyword? x)))

(defn wrap-value-enum
  "Ensure that x is an owned type in Rust.
   If the type t is :any then x is a Kalai BValue so wrap it in an r/value."
  [t x]
  (let [wrap-owned-expression (if (literal? x)
                                x
                                (clone x))]
    (if (= t :any)
      (list 'r/value wrap-owned-expression)
      wrap-owned-expression)))

;;
;; symbol -> string fns (refactored from e-string)
;;

(defn identifier
  "For Rust, do a lowercase snake-case, unless it is already uppercased
  (ex: struct or type name), in which case, just return as-is."
  [s]
  (let [s-str (str s)
        id-first-char (first s-str)]
    (if (Character/isUpperCase ^char id-first-char)
      s-str
      (let [snake-case (u/->snake_case s-str)]
        (if (= \_ (first s-str))
          (str \_ snake-case)
          snake-case)))))

(defn fully-qualified-function-identifier-str
  "Take a fully qualified Clojure var representing a function (`function-name`) and convert it
  into the target language appropriate version of a fully qualified function name.
  The target language will have notation for separating segments of the fully qualified
  name (`class-function-separator`).
  If the function var is in the `clojure.lang.RT` namespace, then return the base name
  segment as the function name, qualified by `kalai::`
  (ex: `clojure.lang.RT/get` -> `kalai::get`)."
  [function-name class-function-separator]
  (if (string? function-name)
    function-name
    (let [varmeta (some-> function-name meta :var meta)
          name-of-ns (namespace function-name)]
      (cond

        (= "clojure.lang.RT" name-of-ns)
        (str "kalai::" (name function-name))

        ;; If the function being transpiled is either from Kalai or the user, we assume it has a namespace.
        ;; We need to handle the Rust snake-casing segment-by-segment when applying `identifier`.
        name-of-ns
        (let [fn-ns-name (if varmeta
                           (str (:ns varmeta))
                           name-of-ns)
              clojure-ns-by-dot (str/split fn-ns-name #"\.")
              rustified-ns-by-dot (map identifier clojure-ns-by-dot)
              rustified-ns (str/join "." rustified-ns-by-dot)
              fn-base-name (if varmeta
                             (:name varmeta)
                             (name function-name))]
          (str "crate::"
               (str/replace rustified-ns "." "::") ;; we use varmeta because we want the full ns, not an alias
               class-function-separator (identifier fn-base-name)))

        ;; This path is expected to be run for user-specific external Rust crate symbols that come from
        ;; hard-coded dependencies. In such a case, we assume that the user has provided the fully-qualified
        ;; Rust symbol from the dependency as a string, ex: `"external_dep::module::fn_symbol"`.
        :else
        (str (identifier function-name))))))
