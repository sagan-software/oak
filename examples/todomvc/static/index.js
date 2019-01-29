require("./index.css");
import("../pkg/todomvc")
  .then(pkg => pkg.main())
  .catch(console.error);
