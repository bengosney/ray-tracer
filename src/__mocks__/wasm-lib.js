class Vec3 {
  constructor(x, y, z) {
    this.x = x;
    this.y = y;
    this.z = z;
  }
}

class Rgb {
  constructor(r, g, b) {
    this.r = r;
    this.g = g;
    this.b = b;
  }
}

module.exports = { Vec3, Rgb };
