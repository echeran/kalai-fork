(ns kalai.pass.java.util
  (:require [clojure.string :as string]
            [clojure.string :as str]
            [camel-snake-kebab.core :as csk]
            [kalai.util :as u]
            [kalai.types :as t])
  (:import (clojure.lang IMeta)))

(defn java-case
  "Return the identifier back as the same type it is provided as (string, symbol), but
  using lowercase camelcase.
  Also, sanitize the identifer by removing question marks (`?`), etc."
  [identifier]
  (cond-> (str/replace
            (if (str/includes? (str identifier) "-")
              (csk/->camelCase identifier)
              identifier)
            "?"
            "")

          (symbol? identifier)
          symbol))

(defn fully-qualified-function-identifier-str [function-name class-function-separator]
  (if (string? function-name)
    function-name
    (let [varmeta (some-> function-name meta :var meta)
          name-of-ns (namespace function-name)]
      (cond

        (or (= "clojure.lang.RT" name-of-ns)
            (= 'clojure.core (some-> varmeta :ns ns-name)))
        (str "kalai.Kalai." (java-case (name function-name)))

        (and (str/includes? (str function-name) "/") varmeta)
        (let [s (str (:ns varmeta))
              xs (str/split s #"\.")
              packagename (str/join "." (for [z (butlast xs)]
                                          (str/lower-case (java-case z))))
              classname (csk/->PascalCase (last xs))
              full-classname (str packagename "." classname)
              function-name (java-case (str (:name varmeta)))]
          (str full-classname class-function-separator function-name))

        :else
        (java-case function-name)))))

contains?

;; TODO: simplify this
(defn get-type
  "Return the Kalai type representation for the expression"
  [expr]
  (if (instance? IMeta expr)
    ;; We expect that the upstream "kalai" non-target language specific pipeline
    ;; will have already normalized the type information into the `:t` key of metadata,
    ;; by copying over and translating type hints from `:tag` or `:type` into the
    ;; equivalent Kalai type keywords. Therefore, only look at `:t`.
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

