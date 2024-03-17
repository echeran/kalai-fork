(ns data-diff.core)

(declare diff)

(defn diff-associative-key
  "Diff associative things a and b, comparing only the key k."
  [a b k]
  (let [va (get a k)
        vb (get b k)
        [aa bb ab] (diff va vb)
        ^boolean in-a (contains? a k)
        ^boolean in-b (contains? b k)
        ^boolean d (not ^boolean (nil? ab))
        ^boolean c  (or d
                        ^boolean (and ^boolean (nil? va)
                                      ^boolean (nil? vb)))
        ^boolean same (and in-a
                           in-b
                           c)
        ^boolean e (not (nil? aa))
        ^boolean f (not (nil? bb))
        ^boolean g (or e ^boolean (not same))
        ^boolean h (or f ^boolean (not same))]
    ^{:t {:vector [:any]} :cast :any}
       [(when (and in-a g) ^{:cast :any} {k aa})
        (when (and in-b h) ^{:cast :any} {k bb})
        (when same ^{:cast :any} {k ab})]))

(defn merge2
  "A helper function to replace `merge` with `(reduce conj...)`"
  [m1 m2]
  ;; TODO: observe change to our `(seq...)` matching rule from `(j/method 'stream ?coll)` to `(j/invoke seq ?coll)`
  ;; TODO: update Java b_function rule for `reduce` to match Rust corresponding rule to create lambda around `conj` fn arg to HOF reduce
  (reduce conj m1 (seq m2)))

(defn merge-diffs [^{:t :any} diff1 ^{:t :any} diff2]
  ;; TODO: move the explict seq calls into function_call
  ^{:cast :any} (vec (map merge2 (seq diff1) (seq diff2))))

(defn diff-associative
  "Diff associative things a and b, comparing only keys in ks."
  [a b ks]
  (reduce
    merge-diffs
    ^{:t {:vector [:any]} :cast :any} [nil nil nil]
    (map (fn [k]
           (diff-associative-key a b k))
         (seq ks))))

;;(defn- diff-sequential
;;  [a b]
;;  (vec (map vectorize (diff-associative
;;                        (if (vector? a) a (vec a))
;;                        (if (vector? b) b (vec b))
;;                        (range (max (count a) (count b)))))))

;;(equality-partition [^Object x]
;;  (if (.. x getClass isArray) :sequential :atom))

(defn union
  [s1 s2]
  (if (< (count s1) (count s2))
    (reduce conj s2 (seq s1))
    (reduce conj s1 (seq s2))))

(defn difference
  "Return a set that is the first set without elements of the remaining sets"
  [s1 s2]
  (if (< (count s1) (count s2))
    (reduce (fn [result item]
              (if (contains? s2 item)
                (disj result item)
                result))
            s1 (seq s1))
    (reduce disj s1 (seq s2))))

(defn intersection
  "Return a set that is the intersection of the input sets"
  [s1 s2]
  (if (< (count s2) (count s1))
    (intersection s2 s1)
    (reduce (fn [result item]
              (if (contains? s2 item)
                result
                (disj result item)))
            s1 (seq s1))))

;; any input must be one of: atom set map sequence

(defn- atom-diff
  "Internal helper for diff."
  [a b]
  (if (= a b)
    ^{:t {:vector [:any]} :cast :any} [nil nil a]
    ^{:t {:vector [:any]} :cast :any} [a b nil]))

(defn equality-partition ^String [x]
  (cond (set? x) :set
        (map? x) :map
        (vector? x) :sequence
        :else :atom))

(defn map-diff [a b]
  (let [ab-keys (union (keys a) (keys b))]
    (diff-associative a b ab-keys)))

(defn set-diff [a b]
  ^{:t {:vector [:any]} :cast :any}
  [(difference a b)
   (difference b a)
   (intersection a b)])

(defn- vectorize
  "Convert an associative-by-numeric-index collection into
   an equivalent vector, with nil for any missing keys"
  [m]
  (when (not-empty m)
    (reduce
      (fn [result [k v]] (assoc result k v))
      ^{:cast :any}
      (vec (repeat ^{:cast :long :t :any} (reduce (fn [a b]
                                                    (let [a-int ^{:cast :long} a
                                                          b-int ^{:cast :long} b]
                                                      ^{:cast :any} (max a-int b-int)))
                                                  (first (seq (keys m)))
                                                  (seq (keys m)))
                   nil))
      (seq m))))

(defn sequence-diff [a b]
  ^{:cast :any}
  (vec (map vectorize (seq (diff-associative
                             (if (vector? a) a ^{:cast :any} (vec (seq a)))
                             (if (vector? b) b ^{:cast :any} (vec (seq b)))
                             ^{:cast :any} (vec (range (max (count a) (count b)))))))))

(defn diff-similar [a b]
  (let [^String partition-a (equality-partition a)
        ^String partition-b (equality-partition b)]
    (if (= partition-a partition-b)
      (cond
        (= partition-a :set) (set-diff a b)
        (= partition-a :map) (map-diff a b)
        (= partition-a :sequence) (sequence-diff a b)
        (= partition-a :atom) (atom-diff a b))
      (atom-diff a b))))

(defn diff
  "Recursively compares a and b, returning a tuple of
  [things-only-in-a things-only-in-b things-in-both].
  Comparison rules:

  * For equal a and b, return [nil nil a].
  * Maps are subdiffed where keys match and values differ.
  * Sets are never subdiffed.
  * All sequential things are treated as associative collections
    by their indexes, with results returned as vectors.
  * Everything else (including strings!) is treated as
    an atom and compared for equality."
  {:added "1.3"}
  [^{:t :any} a, ^{:t :any} b]
  (if (= a b)
    ^{:t {:vector [:any]} :cast :any}
    [nil nil a]
    (diff-similar a b)))
