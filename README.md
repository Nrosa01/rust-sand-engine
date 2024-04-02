Como esto está compilado de forma totalmente dinámica, es necesario copiar std-e493bcbfdc66a475.dll a la carpeta de salida de la aplicación.

Es un poco incómodo pero no hay otra forma, ya que si se compilase de forma estática el tamaño del ejecutable sería mucho mayor, además de que falla por tener
los mismos símbolos en diferentes archivos.

Es obligatorio compilar con rustc 1.76.0 (07dca489a 2024-02-04), ya que el ABI de Rust no es estable