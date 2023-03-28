(defun is-prime (n)
  (when (> n 1)
    (loop for i from 2 to (isqrt n) never (zerop (mod n i)))))

(defun prime-factor-decomposition (&optional (n 600851475143)))