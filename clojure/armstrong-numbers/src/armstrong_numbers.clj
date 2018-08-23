(ns armstrong-numbers)

(defn- digit-seq [num]
  (cond (= num 0) [0]
        (not (integer? num)) []
        (pos? num) (->> num
                        (iterate #(quot % 10))
                        (take-while pos?)
                        (mapv #(mod % 10)))
        :else []))

(defn- pow [base exp]
  (loop [n exp, y (num 1), z base]
    (let [t (even? n), n (quot n 2)]
         (cond t (recur n y (* z z))
               (zero? n) (* z y)
               :else (recur n (* z y) (* z z))))))

(defn armstrong? [num]
  (let [digits (digit-seq num)
        num-digits (count digits)]
    (= num (reduce #(+ %1 (pow %2 num-digits)) 0 digits))))
