(ns data-diff.examples
  (:require [data-diff.core :as d]))

(defn -main ^{:t :void} [& _args]
      (let [a ^{:t {:vector [:long]}} [1 2]
            b ^{:t {:vector [:long]}} [1 3]
            v (d/diff a b)]
           (println v))
      )
