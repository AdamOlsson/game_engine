# Conservation of Momentum
The law of conservation of momentum states that the total momentum of the system before the collision must remain the same as total momentum after the collision. Here $p_{pre}^A$ and $p_{post}^A$ is the combined linear and angular momentum of object A before and after the collision, the same goes for object B.
$$
p_{pre}^A + p_{pre}^B = p_{post}^A + p_{post}^B 
$$

When expanding to multiple dimensions, we preserve the momentum in each dimension independently.

The momentum of an object can be expanded to:
$$
p_{linear} = mv
$$
$$
p_{angular} = I\omega = r \times p_{linear}
$$
$$
p = p_{linear} + p_{angular}
$$

where $v$ is the velocity and $m$ is the mass of the object.


# Impulse Magnitude
We need to compute the total impulse going into the collision. The intuition is that imagine two objects collide at point $P$, both object contribute with linear and angular force into the contact point. We calculate the impact through the following equation 
$$
J = \frac{-(1 + e)v_{pre}^{relative} \cdot n}
{n \cdot n(\frac{1}{m^A} + \frac{1}{m^B}) + \frac{(r^{AP} \times n)^2}{I^A} + \frac{(r^{BP} \times n)^2}{I^B}}
$$
where 
$$ e := \text{coefficient of restituion} $$
$$ P := \text{contact point between the objects} $$
$$ v_{pre}^{relative} := \text{the relative velocity between objects $A$ and $B$ at contact point $P$ before the collision} $$
$$ n := \text{collision normal pointing towards object $A$} $$
$$ I := \text{inertia} $$
$$ r^{AP} := \text{vector from object $A$ center of rotation to collision point $P$} $$

The equation for the relative velocites at the contact point looks as following where $r_{\perp}^{AP}$ is the vector perpendicular to $r^{AP}$ and $\omega$ is the angular velocity:
$$v_{pre}^{AP} = v_{linear}^A + \omega_{pre} r_{\perp}^{AP}$$
$$v_{pre}^{BP} = v_{linear}^B + \omega_{pre} r_{\perp}^{BP}$$
$$v_{pre}^{relative} = v_{pre}^{AP} - v_{pre}^{BP}$$

Once we know the impulse we can insert it into the following equations to get the new linear and angular velocities:
$$v_{post} = v_{pre} + \frac{J}{m}n$$
$$\omega_{post} = \omega_{pre} + \frac{r_{\perp}^{AP} \cdot j n}{I}$$

<!-- Because we apply an impulse at point $P$, the impulse will produce a change in linear and angular velocity. Note that only the latter term in the above two equations uses the impulse magnitute. -->
<!-- $$v_{\Delta} = \frac{J}{m}n$$ -->
<!-- $$\omega_{\Delta} = \frac{r_{\perp}^{AP} \cdot j n}{I}$$ -->

# Example Linear Elastic Collision 1D
A circle collides with a rectangle at rest. For consitency with the other examples, we will use vectors for this example but still only consider the x-axis.

$$m^{circle} = m^{rectangle} = 1.0$$ 
$$v^{circle} = [7.0, 0.0]$$
$$v^{rectangle} = [0.0, 0.0]$$
$$P = [-5.0, 0.0]$$
$$e = 1.0 $$
$$n = [-1.0, 0.0] \text{  (pointing towards the circle)}$$

Because we in this exmaple do not consider angular effects, the impulse magnitude and relative velocity can be simplified: 
$$
v_{pre}^{relative} = v_{pre}^{AP} - v_{pre}^{BP} = v_{linear}^A - v_{linear}^B
$$
$$
J = \frac{-(1 + e)v_{pre}^{relative} \cdot n}
{n \cdot n(\frac{1}{m^A} + \frac{1}{m^B}) + \frac{(r^{AP} \times n)^2}{I^A} + \frac{(r^{BP} \times n)^2}{I^B}} = \frac{-(1 + e)v_{pre}^{relative} \cdot n}{n \cdot n(\frac{1}{m^A} + \frac{1}{m^B})}
$$
Inserting all the values:
$$
v_{pre}^{relative} = v_{pre}^{AP} - v_{pre}^{BP} =[7.0,0.0] - [0.0,0.0] = [10.0,0.0]
$$
$$
J = \frac{-(1 + e)v_{pre}^{relative} \cdot n}{n \cdot n(\frac{1}{m^A} + \frac{1}{m^B})}
= \frac{-(1.0 + 1.0)[7.0,0.0] \cdot [-1.0,0.0]}{[-1.0, 0.0] \cdot [-1.0,0.0](\frac{1}{1} + \frac{1}{1})}
= \frac{-(2.0)(-7.0)}{1.0(2.0)}
= \frac{14.0}{2}
= 7.0
$$

Calculating the new velocites:
$$v_{post}^{circle} = v_{pre}^{circle} + \frac{J}{m^{circle}}n 
= [7.0,0] + \frac{7.0}{1.0}[-1.0,0] 
= [7.0,0.0] + [-7.0, 0.0]
= [0.0,0.0]
$$
$$v_{post}^{rectangle} = v_{pre}^{rectangle} + \frac{-J}{m^{rectangle}}n 
= [0.0,0.0] + \frac{-7.0}{1.0}[-1.0,0] 
= [7.0,0.0] - [7.0, 0.0]
= [7.0,0.0]
$$
Trivially, we can also verify that the moment has been conserved
$$p_{pre}^{circle} + p_{pre}^{rectangle} = p_{post}^{circle} + p_{post}^{rectangle}  \rightarrow$$
$$m^{circle}v_{pre}^{circle} + m^{rectangle}v_{pre}^{rectangle} = m^{circle}v_{post}^{circle} + m^{rectangle}v_{post}^{rectangle}  \rightarrow$$
$$1.0[7.0,0.0] + 1.0[0.0,0.0] = 1.0[0.0,0.0] + 1.0[7.0,0.0] \rightarrow $$
$$1.0(\sqrt{7.0^2 + 0.0^2}) = 1.0(\sqrt{7.0^2 + 0.0^2}) = \rightarrow $$
$$ 7.0 = 7.0$$

# Example Linear Elastic Collision 2D
Now we take the same example as above but add a y-component. Here consider a circle colliding with a rectangle at rest, however the circle is moving diagonally.
$$m^{circle} = m^{rectangle} = 1.0$$ 
$$v^{circle} = [7.0, -7.0]$$
$$v^{rectangle} = [0.0, 0.0]$$
$$P = [-5.0, 0.0]$$
$$e = 1.0 $$
$$n = [-1.0, 0.0] \text{  (pointing towards the circle)}$$

We neither consider angular effects as the collision point $P$ is align with the rectangles center of mass.
$$v_{pre}^{relative}
= v_{linear}^{circle} - v_{linear}^{rectangle}
= [7.0, -7.0] - [0.0,0.0]
= [-7.0, 7.0]$$
$$J=\frac{-(1.0 + e)v_{pre}^{relative} \cdot n}{n \cdot n(\frac{1.0}{m^A} + \frac{1.0}{m^B})}
=\frac{-(1.0 + 1.0)[7.0,-7.0] \cdot [-1.0,0.0]}{[-1.0,0.0] \cdot [-1.0,0.0](\frac{1.0}{1.0} + \frac{1.0}{1.0})}
=\frac{-2.0(-7.0)}{2.0}
= 7.0
$$

Calculating the new velocities
$$v_{post}^{circle} = v_{pre}^{circle} + \frac{J}{m^{circle}}n 
= [7.0,-7.0] + \frac{7.0}{1.0}[-1.0, 0.0] 
= [7.0,-7.0] + [-7.0, 0.0]
= [0.0,-7.0]
$$
$$v_{post}^{rectangle} = v_{pre}^{rectangle} + \frac{-J}{m^{rectangle}}n 
= [0.0,0.0] + \frac{-7.0}{1.0}[-1.0,0] 
= [7.0,0.0] - [7.0, 0.0]
= [7.0,0.0]
$$

Validating that momentum has been preserved in the x-component

$$p_{pre_{x}}^{circle} + p_{pre_{x}}^{rectangle} = p_{post_{x}}^{circle} + p_{post_{x}}^{rectangle}  \rightarrow$$
$$m^{circle}v_{pre_{x}}^{circle} + m^{rectangle}v_{pre_{x}}^{rectangle} = m^{circle}v_{post_{x}}^{circle} + m^{rectangle}v_{post_{x}}^{rectangle}  \rightarrow$$
$$1.0*7.0 + 1.0*0.0 = 1.0*0.0 + 1.0*7.0 \rightarrow $$
$$ 7.0 = 7.0$$

and similarly for the y-component

$$p_{pre_{y}}^{circle} + p_{pre_{y}}^{rectangle} = p_{post_{y}}^{circle} + p_{post_{y}}^{rectangle}  \rightarrow$$
$$m^{circle}v_{pre_{y}}^{circle} + m^{rectangle}v_{pre_{y}}^{rectangle} = m^{circle}v_{post_{y}}^{circle} + m^{rectangle}v_{post_{y}}^{rectangle}  \rightarrow$$
$$1.0*-7.0 + 1.0*0.0 = 1.0*-7.0 + 1.0*0.0 \rightarrow $$
$$ -7.0 = -7.0$$
# Sources
- https://chrishecker.com/Rigid_Body_Dynamics
- https://chrishecker.com/images/c/c2/Gdmphys2.pdf
- https://chrishecker.com/images/e/e7/Gdmphys3.pdf
- https://phys.libretexts.org/Courses/Prince_Georges_Community_College/PHY_1030%3A_General_Physics_I/07%3A_Linear_Momentum_and_Collisions/7.3%3A_Collisions
- https://www.sparknotes.com/physics/linearmomentum/collisions/section2/
- https://www2.tntech.edu/leap/murdock/books/v1chap7.pdf
- https://research.ncl.ac.uk/game/mastersdegree/gametechnologies/physicstutorials/5collisionresponse/Physics%20-%20Collision%20Response.pdf