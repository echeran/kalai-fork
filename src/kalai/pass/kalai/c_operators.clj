(ns kalai.pass.kalai.c-operators
  (:require [kalai.util :as u]
            [meander.strategy.epsilon :as s]
            [meander.epsilon :as m]))

(def rewrite
  (s/bottom-up
    (s/rewrite
      ;; not really an operator, but seems like it belongs here
      (invoke clojure.lang.RT/intCast ?x)
      ~(if (number? ?x)
         (clojure.lang.RT/intCast ?x)
         ?x)

      ;; binary operators
      (invoke (m/pred u/binary-operator ?op) ?x ?y)
      (operator (m/app u/binary-operator ?op) ?x ?y)

      ;; unitary operators
      (invoke not ?x)
      (operator '! ?x)

      ;; Direct calls of inc/dec are treated as operator +1 with the type transfered to the literal 1
      ;; For when used as a function value see lang/function_call
      (m/or
        (invoke (u/var ~#'inc) ?x)
        (invoke clojure.lang.Numbers/inc ?x)
        (invoke clojure.lang.Numbers/unchecked_inc ?x))
      (operator + ?x ~(if (= :int (:t (meta ?x)))
                        (int 1)
                        1))

      (m/or
        (invoke (u/var ~#'dec) ?x)
        (invoke clojure.lang.Numbers/dec ?x)
        (invoke clojure.lang.Numbers/unchecked_dec ?x))
      (operator - ?x ~(if (= :int (:t (meta ?x)))
                        (int 1)
                        1))

      ;; varity operators
      (invoke + . !args ...)
      (operator + . !args ...)

      (invoke - . !args ...)
      (operator - . !args ...)

      (invoke and)
      true

      (invoke and . !args ...)
      (operator && . !args ...)

      (invoke or)
      false

      (invoke or . !args ...)
      (operator || . !args ...)

      ;;;

      ;;(invoke (u/var ~#'println) & ?more)
      ;;(invoke System.out.println & ?more)


      ?else
      ?else)))
