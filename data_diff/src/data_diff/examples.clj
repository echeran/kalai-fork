(ns data-diff.examples
  (:require [data-diff.core :as d]))

(defn -main ^{:t :void} [& _args]
      (let [a ^{:t {:vector [:any]}, :cast :any} [1 2]
            b ^{:t {:vector [:any]}, :cast :any} [1 3]
            v (d/diff a b)]
           (println v))
      )
