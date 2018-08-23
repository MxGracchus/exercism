(ns reverse-string
  (:import java.text.BreakIterator))

(defn- break-points [s]
  (let [iter (doto (BreakIterator/getCharacterInstance) (.setText s))]
    (vec (->> iter
              (partial (memfn next))
              repeatedly
              (take-while #(not= BreakIterator/DONE %))))))

(defn- break-string
  ([s]
   (let [points (break-points s)]
     (if (empty? points) ""
         (break-string s 0 (first points) (rest points)))))
  ([s start end points]
     (cons (subs s start end)
           (if (empty? points) ""
               (break-string s end (first points) (rest points))))))

(defn reverse-string [s] ;; <- arglist goes here
  (apply str (reverse (break-string s))))
