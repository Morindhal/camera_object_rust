static RESOLUTION: [(i32, i32); 17] = 
    [
        (160, 90),
        (160, 100),
        (160, 120),
        (176, 144),
        (240, 180),
        (320, 180),
        (320, 200),
        (320, 240),
        (480, 270),
        (480, 300),
        (480, 360),
        (640, 360),
        (640, 400),
        (640, 480),
        (800, 450),
        (800, 500),
        (800, 600),
    ];
static FRAMERATE: [i32; 15] = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30];
static QUALITY: [i32; 16] = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80];


fn iterate_over_array(target: i32, array: &[i32])
    -> i32
{
    let mut best_fit = 0;
    if !array.is_empty()
    {
        best_fit = array[0]
    }
    for &val in array.iter()
    {
        if target >= val
        {
            best_fit = val;
        }
    }
    best_fit
}

fn correct_camera_object(camera_object: &mut ((i32, i32), i32, i32))
{
    {
        let mut best_fit = (0,0);
        for &(width, height) in RESOLUTION.iter()
        {
            if (camera_object.0).0 >= width && (camera_object.0).1 >= height
            {
                best_fit.0 = width;
                best_fit.1 = height;
            }
        }
        (camera_object.0).0 = best_fit.0;
        (camera_object.0).1 = best_fit.1;
    }
    camera_object.1 = iterate_over_array(camera_object.1, &FRAMERATE);
    camera_object.2 = iterate_over_array(camera_object.2, &QUALITY);
}

fn main()
{
    let mut camera_object = ((780, 450), 200, 200);
    println!("{:?}", camera_object);
    correct_camera_object(&mut camera_object);
    println!("{:?}", camera_object);
    correct_camera_object(&mut camera_object);
}
