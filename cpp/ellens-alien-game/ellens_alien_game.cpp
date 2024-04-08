namespace targets {
    class Alien {
        private:
            int health;
        public:
            int x_coordinate{0};
            int y_coordinate{0};
            Alien(int x, int y): x_coordinate{x}, y_coordinate {y}{
                health = 3;
            }
            ~Alien(){}
            int get_health() {
                return health;
            }

            bool hit() {
                if (health) {
                    --this->health;
                }
                return true;
            }

            bool is_alive() {
                return !(health == 0);
            }

            bool teleport(int xn, int yn) {
                x_coordinate = xn;
                y_coordinate = yn;
                return true;
            };

            bool collision_detection(Alien other) {
                return x_coordinate == other.x_coordinate &&
                    y_coordinate == other.y_coordinate;
            }
    };

}
