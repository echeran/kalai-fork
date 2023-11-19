(ns kalai.pass.java.util
  (:require [clojure.string :as str]
            [camel-snake-kebab.core :as csk]
            [kalai.util :as u]
            [kalai.types :as t])
  (:import (clojure.lang IMeta)))

(defn fully-qualified-function-identifier-str [function-name class-function-separator]
  (if (string? function-name)
    function-name
    (let [varmeta (some-> function-name meta :var meta)
          name-of-ns (namespace function-name)]
      (cond

        (= "clojure.lang.RT" name-of-ns)
        (str "kalai.Kalai." (name function-name))

        (and (str/includes? (str function-name) "/") varmeta)
        (let [s (str (:ns varmeta))
              xs (str/split s #"\.")
              packagename (str/join "." (for [z (butlast xs)]
                                          (str/lower-case (csk/->camelCase z))))
              classname (csk/->PascalCase (last xs))
              full-classname (str packagename "." classname)
              function-name (csk/->camelCase (str (:name varmeta)))]
          (str full-classname class-function-separator function-name))

        (str/includes? function-name "-")
        (csk/->camelCase function-name)

        :else
        function-name))))

;; TODO: simplify this
(defn get-type
  "Return the Kalai type representation for the expression"
  [expr]
  (if (instance? IMeta expr)
    (let [{:keys [t]} (meta expr)]
      (or t
          (when (and (seq? expr) (seq expr))
            (case (first expr)
              ;; TODO: this suggests we need some type inference
              (j/new) (second expr)
              (j/block j/invoke do if) (get-type (last expr))
              :any))
          (when (not (symbol? expr))
            (type expr))
          :any))
    (t/get-kalai-type-from-java-type (type expr))))

(defn tmp-for [expr]
  (u/tmp (get-type expr) expr))

