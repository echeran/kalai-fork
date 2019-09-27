(ns clj-icu-test.cpp.cpp-types-test
  (:require [clj-icu-test.common :refer :all]
            [clj-icu-test.emit.api :refer :all]
            [clj-icu-test.emit.langs :as l]
            [clojure.tools.analyzer.jvm :as az]
            [expectations.clojure.test :refer :all])
  (:import clj_icu_test.common.AstOpts))


(reset-indent-level)

(defexpect vectors
  (do
    (import 'java.util.List)
    (let [ast (az/analyze '(def ^{:mtype [List [Integer]]} numbers [13 17 19 23]))]
      (expect "std::vector<int> numbers = {13, 17, 19, 23};"
              (emit (map->AstOpts {:ast ast :lang ::l/cpp}))))
    (let [ast (az/analyze '(do (def x 13) (def ^{:mtype [List [Integer]]} numbers [x])))]
      (expect ["x = 13;"
               "std::vector<int> numbers = {x};"]
              (emit (map->AstOpts {:ast ast :lang ::l/cpp}))))
    (let [ast (az/analyze '(def ^{:mtype [List [List [Integer]]]} matrix [[2 3 5]
                                                                          [7 11 13]
                                                                          [17 19 23]]))]
      (expect ["std::vector<int> matrixV0 = {2, 3, 5};"
               "std::vector<int> matrixV1 = {7, 11, 13};"
               "std::vector<int> matrixV2 = {17, 19, 23};"
               "std::vector<std::vector<int>> matrix = {matrixV0, matrixV1, matrixV2};"]
              (emit (map->AstOpts {:ast ast :lang ::l/cpp}))))))

(defexpect maps
  ;; (let [ast (az/analyze '{"one" 1
  ;;                         "two" 2
  ;;                         "three" 3})]
  ;;   )
  (do
    (import 'java.util.Map)
    (let [ast (az/analyze '(def ^{:mtype [Map [String Integer]]} number-words {"one" 1
                                                                               "two" 2
                                                                               "three" 3}))]
      "TODO")))

(defexpect sets
  (do
    (import 'java.util.Set)
    ;; (let [ast (az/analyze '#{"smell" "sight" "touch" "taste" "hearing"})]
    ;;   )
    (let [ast (az/analyze '(def ^{:mtype [Set [String]]} senses #{"smell" "sight" "touch" "taste" "hearing"}))]
      "TODO")))

(defexpect coll-type-nested
  (do
    (import '[java.util List Map Set])
    (let [ast (az/analyze '(def ^{:mtype [Map [String List [Character]]]} number-systems-map {}))]
      "TODO")))
