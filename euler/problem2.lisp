(defun fibonacci (&optional (limit 4e6))
  (let ((ret nil))
    (do ((b 1 a)
         (a 1 (incf a b)))
        ((>= a limit)
         (return-from fibonacci
          (reduce #'+ (remove-if #'oddp (reverse ret)))))
        (push a ret))))