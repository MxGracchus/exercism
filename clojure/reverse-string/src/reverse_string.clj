(ns reverse-string
  (:import java.text.BreakIterator))

(defn- break-points [s]
    (vec (->> (doto (BreakIterator/getCharacterInstance) (.setText s))
              (partial (memfn next))
              repeatedly
              (take-while #(not= BreakIterator/DONE %)))))

(defn- break-string
  ([s]
   (let [points (break-points s)]
     (if (empty? points) ""
         (break-string s 0 (first points) (rest points)))))
  ([s start end points]
     (cons (subs s start end)
           (if (empty? points) ""
               (break-string s end (first points) (rest points))))))

(defn- rev [xs]
  (if (empty? xs) xs
      (concat (rev (rest xs)) (first xs))))

(defn reverse-string [s] ;; <- arglist goes here
  (apply str (rev (break-string s))))
