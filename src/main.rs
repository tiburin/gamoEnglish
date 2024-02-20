use gamo::apps::Sphere;
static WRITE_BUILD: bool = false;
static WRITE_BOOKTORE: bool = false;

fn main() {
    // put line below in the empty input
    // aparter
    let sphere = Sphere::new().setup("aparter");
    if WRITE_BOOKTORE {
        gamo::write_booktore(&sphere)
    }
    if WRITE_BUILD {
        gamo::write_build(&sphere, false);
    }
}
