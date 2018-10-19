import UIKit

class ViewController: UIViewController {
    
    @IBOutlet var actionLabel: UILabel?
    
    var session = session_new()

    override func viewDidLoad() {
        super.viewDidLoad()
        let string = session_action(self.session!, "Klaus")!.takeRetainedValue()
        let s = string as NSString
        self.actionLabel?.text = s as String
    }

}

